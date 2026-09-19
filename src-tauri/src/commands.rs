use crate::db::DbState;
use crate::models::*;
use crate::paths;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use rusqlite::{params, params_from_iter, Connection, OptionalExtension};
use std::collections::HashMap;
use tauri::State;

type CmdResult<T> = Result<T, String>;

fn map_err<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

/// Ne garde que des caractères sûrs pour un nom de fichier, en conservant l'extension.
fn sanitize_filename(name: &str) -> String {
    let base = name.rsplit(['/', '\\']).next().unwrap_or(name);
    let cleaned: String = base
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if cleaned.is_empty() {
        "fichier".to_string()
    } else {
        cleaned
    }
}

fn timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn write_upload(file_name: &str, base64_data: &str) -> CmdResult<String> {
    let bytes = STANDARD.decode(base64_data).map_err(map_err)?;
    let stored_name = format!("{}_{}", timestamp(), sanitize_filename(file_name));
    let dest = paths::uploads_dir().join(&stored_name);
    std::fs::write(&dest, &bytes).map_err(map_err)?;
    Ok(stored_name)
}

fn delete_upload_if_exists(stored_name: &str) {
    if stored_name.is_empty() {
        return;
    }
    let path = paths::uploads_dir().join(stored_name);
    let _ = std::fs::remove_file(path);
}

// ══════════════════════════════════════════════════════════════
// ARBORESCENCE + CHARGEMENT GLOBAL
// ══════════════════════════════════════════════════════════════

#[tauri::command]
pub fn get_all_data(state: State<DbState>) -> CmdResult<AllData> {
    let conn = state.0.lock().map_err(|_| "verrou base indisponible".to_string())?;

    let mut data = AllData::default();

    let mut stmt = conn.prepare("SELECT id_classeur, titre FROM classeurs").map_err(map_err)?;
    data.classeurs = stmt
        .query_map([], |r| Ok(Classeur { id_classeur: r.get(0)?, titre: r.get(1)? }))
        .map_err(map_err)?
        .collect::<Result<_, _>>()
        .map_err(map_err)?;

    let mut stmt = conn
        .prepare("SELECT id_sous_classeur, id_classeur, titre FROM sous_classeurs")
        .map_err(map_err)?;
    data.sous_classeurs = stmt
        .query_map([], |r| {
            Ok(SousClasseur { id_sous_classeur: r.get(0)?, id_classeur: r.get(1)?, titre: r.get(2)? })
        })
        .map_err(map_err)?
        .collect::<Result<_, _>>()
        .map_err(map_err)?;

    let mut stmt = conn
        .prepare("SELECT id_s_s_classeur, id_sous_classeur, titre FROM sous_sous_classeurs")
        .map_err(map_err)?;
    data.sous_sous_classeurs = stmt
        .query_map([], |r| {
            Ok(SousSousClasseur { id_s_s_classeur: r.get(0)?, id_sous_classeur: r.get(1)?, titre: r.get(2)? })
        })
        .map_err(map_err)?
        .collect::<Result<_, _>>()
        .map_err(map_err)?;

    let mut stmt = conn
        .prepare("SELECT id_s_s_s_classeur, id_s_s_classeur, titre FROM sous_sous_sous_classeurs")
        .map_err(map_err)?;
    data.sous_sous_sous_classeurs = stmt
        .query_map([], |r| {
            Ok(SousSousSousClasseur { id_s_s_s_classeur: r.get(0)?, id_s_s_classeur: r.get(1)?, titre: r.get(2)? })
        })
        .map_err(map_err)?
        .collect::<Result<_, _>>()
        .map_err(map_err)?;

    let mut stmt = conn.prepare(FICHE_COLUMNS_SELECT).map_err(map_err)?;
    data.fiches = stmt
        .query_map([], row_to_fiche)
        .map_err(map_err)?
        .collect::<Result<_, _>>()
        .map_err(map_err)?;

    let mut stmt = conn.prepare("SELECT cle, valeur FROM parametres").map_err(map_err)?;
    let params_iter = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?.unwrap_or_default())))
        .map_err(map_err)?;
    let mut params_map: HashMap<String, String> = HashMap::new();
    for row in params_iter {
        let (k, v) = row.map_err(map_err)?;
        params_map.insert(k, v);
    }
    data.parametres = params_map;

    Ok(data)
}

const FICHE_COLUMNS_SELECT: &str = "SELECT id_fiche_produit, id_classeur, id_sous_classeur, id_s_s_classeur, id_s_s_s_classeur,
    titre, descriptif, photo_1, photo_2, synonymes, tags, epidemiologie, physiopathologie,
    clinique, paraclinique, traitement, prevention, bibliographie FROM fiches_produits";

fn row_to_fiche(r: &rusqlite::Row) -> rusqlite::Result<FicheProduit> {
    Ok(FicheProduit {
        id_fiche_produit: r.get(0)?,
        id_classeur: r.get(1)?,
        id_sous_classeur: r.get(2)?,
        id_s_s_classeur: r.get(3)?,
        id_s_s_s_classeur: r.get(4)?,
        titre: r.get(5)?,
        descriptif: r.get(6)?,
        photo_1: r.get(7)?,
        photo_2: r.get(8)?,
        synonymes: r.get(9)?,
        tags: r.get(10)?,
        epidemiologie: r.get(11)?,
        physiopathologie: r.get(12)?,
        clinique: r.get(13)?,
        paraclinique: r.get(14)?,
        traitement: r.get(15)?,
        prevention: r.get(16)?,
        bibliographie: r.get(17)?,
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn add_category(
    state: State<DbState>,
    r#type: String,
    titre: String,
    parent_id: Option<i64>,
) -> CmdResult<()> {
    let type_ = r#type;
    let titre = titre.trim().to_string();
    if titre.is_empty() {
        return Err("Le titre ne peut pas être vide.".to_string());
    }
    let conn = state.0.lock().map_err(|_| "verrou base indisponible".to_string())?;

    match type_.as_str() {
        "classeur" => {
            conn.execute("INSERT INTO classeurs (titre) VALUES (?1)", params![titre])
                .map_err(map_err)?;
        }
        "famille" => {
            let parent = parent_id.ok_or("Classeur parent manquant.")?;
            conn.execute(
                "INSERT INTO sous_classeurs (id_classeur, titre) VALUES (?1, ?2)",
                params![parent, titre],
            )
            .map_err(map_err)?;
        }
        "sous_famille" => {
            let parent = parent_id.ok_or("Famille parente manquante.")?;
            conn.execute(
                "INSERT INTO sous_sous_classeurs (id_sous_classeur, titre) VALUES (?1, ?2)",
                params![parent, titre],
            )
            .map_err(map_err)?;
        }
        "sous_sous_famille" => {
            let parent = parent_id.ok_or("Sous-famille parente manquante.")?;
            conn.execute(
                "INSERT INTO sous_sous_sous_classeurs (id_s_s_classeur, titre) VALUES (?1, ?2)",
                params![parent, titre],
            )
            .map_err(map_err)?;
        }
        _ => return Err("Type de dossier inconnu.".to_string()),
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn rename_category(state: State<DbState>, r#type: String, id: i64, titre: String) -> CmdResult<()> {
    let titre = titre.trim().to_string();
    if titre.is_empty() {
        return Err("Le nom est vide.".to_string());
    }
    let (table, pk) = category_table(&r#type)?;
    let conn = state.0.lock().map_err(|_| "verrou base indisponible".to_string())?;
    conn.execute(
        &format!("UPDATE {table} SET titre = ?1 WHERE {pk} = ?2"),
        params![titre, id],
    )
    .map_err(map_err)?;
    Ok(())
}

fn category_table(type_: &str) -> CmdResult<(&'static str, &'static str)> {
    Ok(match type_ {
        "classeur" => ("classeurs", "id_classeur"),
        "famille" => ("sous_classeurs", "id_sous_classeur"),
        "sous_famille" => ("sous_sous_classeurs", "id_s_s_classeur"),
        "sous_sous_famille" => ("sous_sous_sous_classeurs", "id_s_s_s_classeur"),
        _ => return Err("Type de dossier inconnu.".to_string()),
    })
}

/// Récupère tous les chemins de fichiers (photo_1, photo_2, médias) pour un
/// ensemble de fiches, avant que la suppression en cascade ne fasse disparaître
/// les lignes correspondantes.
fn collect_file_paths(conn: &Connection, fiche_ids: &[i64]) -> CmdResult<Vec<String>> {
    if fiche_ids.is_empty() {
        return Ok(vec![]);
    }
    let placeholders = fiche_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let mut paths = Vec::new();

    let sql = format!(
        "SELECT photo_1, photo_2 FROM fiches_produits WHERE id_fiche_produit IN ({placeholders})"
    );
    let mut stmt = conn.prepare(&sql).map_err(map_err)?;
    let rows = stmt
        .query_map(params_from_iter(fiche_ids.iter()), |r| {
            Ok((r.get::<_, Option<String>>(0)?, r.get::<_, Option<String>>(1)?))
        })
        .map_err(map_err)?;
    for row in rows {
        let (p1, p2) = row.map_err(map_err)?;
        if let Some(p) = p1 { paths.push(p); }
        if let Some(p) = p2 { paths.push(p); }
    }

    let sql = format!(
        "SELECT chemin FROM fiche_medias WHERE id_fiche_produit IN ({placeholders})"
    );
    let mut stmt = conn.prepare(&sql).map_err(map_err)?;
    let rows = stmt
        .query_map(params_from_iter(fiche_ids.iter()), |r| r.get::<_, String>(0))
        .map_err(map_err)?;
    for row in rows {
        paths.push(row.map_err(map_err)?);
    }

    Ok(paths)
}

fn fiche_ids_for_scope(conn: &Connection, type_: &str, id: i64) -> CmdResult<Vec<i64>> {
    let sql = match type_ {
        "classeur" => {
            "SELECT id_fiche_produit FROM fiches_produits WHERE id_classeur = ?1
             UNION
             SELECT id_fiche_produit FROM fiches_produits WHERE id_sous_classeur IN
                (SELECT id_sous_classeur FROM sous_classeurs WHERE id_classeur = ?1)
             UNION
             SELECT id_fiche_produit FROM fiches_produits WHERE id_s_s_classeur IN
                (SELECT id_s_s_classeur FROM sous_sous_classeurs WHERE id_sous_classeur IN
                    (SELECT id_sous_classeur FROM sous_classeurs WHERE id_classeur = ?1))
             UNION
             SELECT id_fiche_produit FROM fiches_produits WHERE id_s_s_s_classeur IN
                (SELECT id_s_s_s_classeur FROM sous_sous_sous_classeurs WHERE id_s_s_classeur IN
                    (SELECT id_s_s_classeur FROM sous_sous_classeurs WHERE id_sous_classeur IN
                        (SELECT id_sous_classeur FROM sous_classeurs WHERE id_classeur = ?1)))"
        }
        "famille" => {
            "SELECT id_fiche_produit FROM fiches_produits WHERE id_sous_classeur = ?1
             UNION
             SELECT id_fiche_produit FROM fiches_produits WHERE id_s_s_classeur IN
                (SELECT id_s_s_classeur FROM sous_sous_classeurs WHERE id_sous_classeur = ?1)
             UNION
             SELECT id_fiche_produit FROM fiches_produits WHERE id_s_s_s_classeur IN
                (SELECT id_s_s_s_classeur FROM sous_sous_sous_classeurs WHERE id_s_s_classeur IN
                    (SELECT id_s_s_classeur FROM sous_sous_classeurs WHERE id_sous_classeur = ?1))"
        }
        "sous_famille" => {
            "SELECT id_fiche_produit FROM fiches_produits WHERE id_s_s_classeur = ?1
             UNION
             SELECT id_fiche_produit FROM fiches_produits WHERE id_s_s_s_classeur IN
                (SELECT id_s_s_s_classeur FROM sous_sous_sous_classeurs WHERE id_s_s_classeur = ?1)"
        }
        "sous_sous_famille" => {
            "SELECT id_fiche_produit FROM fiches_produits WHERE id_s_s_s_classeur = ?1"
        }
        _ => return Err("Type de dossier inconnu.".to_string()),
    };
    let mut stmt = conn.prepare(sql).map_err(map_err)?;
    let ids = stmt
        .query_map(params![id], |r| r.get::<_, i64>(0))
        .map_err(map_err)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(map_err)?;
    Ok(ids)
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_category(state: State<DbState>, r#type: String, id: i64) -> CmdResult<()> {
    let (table, pk) = category_table(&r#type)?;
    let conn = state.0.lock().map_err(|_| "verrou base indisponible".to_string())?;

    let fiche_ids = fiche_ids_for_scope(&conn, &r#type, id)?;
    let files = collect_file_paths(&conn, &fiche_ids)?;

    // La suppression en cascade (ON DELETE CASCADE) nettoie toutes les tables
    // filles (sous-dossiers, fiches, médias) automatiquement.
    conn.execute(&format!("DELETE FROM {table} WHERE {pk} = ?1"), params![id])
        .map_err(map_err)?;

    for f in files {
        delete_upload_if_exists(&f);
    }
    Ok(())
}

// ══════════════════════════════════════════════════════════════
// FICHES
// ══════════════════════════════════════════════════════════════

#[tauri::command(rename_all = "snake_case")]
pub fn get_fiche_detail(state: State<DbState>, id: i64) -> CmdResult<FicheDetail> {
    let conn = state.0.lock().map_err(|_| "verrou base indisponible".to_string())?;

    let sql = format!("{FICHE_COLUMNS_SELECT} WHERE id_fiche_produit = ?1");
    let fiche = conn
        .query_row(&sql, params![id], row_to_fiche)
        .optional()
        .map_err(map_err)?
        .ok_or_else(|| "Fiche introuvable.".to_string())?;

    let mut stmt = conn
        .prepare("SELECT id_media, id_fiche_produit, type, chemin, nom_original FROM fiche_medias WHERE id_fiche_produit = ?1 ORDER BY id_media ASC")
        .map_err(map_err)?;
    let medias = stmt
        .query_map(params![id], |r| {
            Ok(FicheMedia {
                id_media: r.get(0)?,
                id_fiche_produit: r.get(1)?,
                type_: r.get(2)?,
                chemin: r.get(3)?,
                nom_original: r.get(4)?,
            })
        })
        .map_err(map_err)?
        .collect::<Result<_, _>>()
        .map_err(map_err)?;

    Ok(FicheDetail { fiche, medias })
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_fiche(state: State<DbState>, payload: UpdateFichePayload) -> CmdResult<()> {
    let conn = state.0.lock().map_err(|_| "verrou base indisponible".to_string())?;
    conn.execute(
        "UPDATE fiches_produits SET titre = ?1, descriptif = ?2, synonymes = ?3,
            id_classeur = ?4, id_sous_classeur = ?5, id_s_s_classeur = ?6, id_s_s_s_classeur = ?7
         WHERE id_fiche_produit = ?8",
        params![
            payload.titre,
            payload.descriptif,
            payload.synonymes,
            payload.id_classeur,
            payload.id_sous_classeur,
            payload.id_s_s_classeur,
            payload.id_s_s_s_classeur,
            payload.id_fiche_produit,
        ],
    )
    .map_err(map_err)?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn upload_fiche(
    state: State<DbState>,
    titre: String,
    descriptif: Option<String>,
    niveau: String,
    destination_id: i64,
    file_name: String,
    file_base64: String,
) -> CmdResult<i64> {
    let stored_name = write_upload(&file_name, &file_base64)?;
    let conn = state.0.lock().map_err(|_| "verrou base indisponible".to_string())?;

    let col = match niveau.as_str() {
        "1" => "id_classeur",
        "2" => "id_sous_classeur",
        "3" => "id_s_s_classeur",
        "4" => "id_s_s_s_classeur",
        _ => return Err("Destination invalide.".to_string()),
    };

    conn.execute(
        &format!("INSERT INTO fiches_produits ({col}, titre, descriptif, photo_1) VALUES (?1, ?2, ?3, ?4)"),
        params![destination_id, titre, descriptif, stored_name],
    )
    .map_err(map_err)?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_fiche(state: State<DbState>, id: i64) -> CmdResult<()> {
    let conn = state.0.lock().map_err(|_| "verrou base indisponible".to_string())?;
    let files = collect_file_paths(&conn, &[id])?;
    conn.execute("DELETE FROM fiches_produits WHERE id_fiche_produit = ?1", params![id])
        .map_err(map_err)?;
    for f in files {
        delete_upload_if_exists(&f);
    }
    Ok(())
}

// ══════════════════════════════════════════════════════════════
// MÉDIAS (galerie images / PDF de la fiche)
// ══════════════════════════════════════════════════════════════

#[tauri::command(rename_all = "snake_case")]
pub fn upload_media(
    state: State<DbState>,
    id_fiche_produit: i64,
    file_name: String,
    file_base64: String,
) -> CmdResult<FicheMedia> {
    let ext = file_name.rsplit('.').next().unwrap_or("").to_lowercase();
    let type_ = match ext.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "webp" => "image",
        "pdf" => "pdf",
        _ => return Err("Format non supporté (image ou PDF uniquement).".to_string()),
    };

    let stored_name = write_upload(&file_name, &file_base64)?;
    let conn = state.0.lock().map_err(|_| "verrou base indisponible".to_string())?;
    conn.execute(
        "INSERT INTO fiche_medias (id_fiche_produit, type, chemin, nom_original) VALUES (?1, ?2, ?3, ?4)",
        params![id_fiche_produit, type_, stored_name, file_name],
    )
    .map_err(map_err)?;

    Ok(FicheMedia {
        id_media: conn.last_insert_rowid(),
        id_fiche_produit,
        type_: type_.to_string(),
        chemin: stored_name,
        nom_original: Some(file_name),
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_media(state: State<DbState>, id_media: i64) -> CmdResult<()> {
    let conn = state.0.lock().map_err(|_| "verrou base indisponible".to_string())?;
    let chemin: Option<String> = conn
        .query_row("SELECT chemin FROM fiche_medias WHERE id_media = ?1", params![id_media], |r| r.get(0))
        .optional()
        .map_err(map_err)?;
    let chemin = chemin.ok_or_else(|| "Média introuvable.".to_string())?;
    conn.execute("DELETE FROM fiche_medias WHERE id_media = ?1", params![id_media])
        .map_err(map_err)?;
    delete_upload_if_exists(&chemin);
    Ok(())
}

/// Lit un fichier du dossier `data/uploads` et le renvoie encodé en data-URL
/// (base64), pour affichage direct dans une balise <img>/<embed>.
#[tauri::command(rename_all = "snake_case")]
pub fn read_media(chemin: String) -> CmdResult<String> {
    let safe_name = sanitize_filename(&chemin);
    let path = paths::uploads_dir().join(&safe_name);
    let bytes = std::fs::read(&path).map_err(|e| format!("Fichier introuvable : {e}"))?;
    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    Ok(format!("data:{};base64,{}", mime, STANDARD.encode(bytes)))
}

/// Chemin absolu d'un fichier de `data/uploads`, pour l'ouvrir avec l'application
/// par défaut du système (plutôt que de faire transiter un data: URL potentiellement
/// volumineux par le mécanisme d'ouverture d'URL externe).
#[tauri::command(rename_all = "snake_case")]
pub fn get_media_path(chemin: String) -> CmdResult<String> {
    let safe_name = sanitize_filename(&chemin);
    let path = paths::uploads_dir().join(&safe_name);
    if !path.exists() {
        return Err("Fichier introuvable.".to_string());
    }
    Ok(path.to_string_lossy().to_string())
}

// ══════════════════════════════════════════════════════════════
// PARAMÈTRES
// ══════════════════════════════════════════════════════════════

#[tauri::command(rename_all = "snake_case")]
pub fn set_parametre(state: State<DbState>, cle: String, valeur: String) -> CmdResult<()> {
    let conn = state.0.lock().map_err(|_| "verrou base indisponible".to_string())?;
    conn.execute(
        "INSERT INTO parametres (cle, valeur) VALUES (?1, ?2)
         ON CONFLICT(cle) DO UPDATE SET valeur = excluded.valeur",
        params![cle, valeur],
    )
    .map_err(map_err)?;
    Ok(())
}

// ══════════════════════════════════════════════════════════════
// EXPORT (téléchargement de fiche en HTML autonome, hors scope fs plugin)
// ══════════════════════════════════════════════════════════════

#[tauri::command(rename_all = "snake_case")]
pub fn export_text_file(path: String, content: String) -> CmdResult<()> {
    std::fs::write(path, content).map_err(map_err)
}
