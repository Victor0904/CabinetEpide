use std::path::PathBuf;
use std::sync::OnceLock;

/// Dossier de données portable : toujours à côté de l'exécutable, jamais dans
/// AppData / Application Support. C'est ce qui permet de glisser l'exécutable
/// + le dossier `data/` dans un .zip et que tout fonctionne au dézippage.
fn portable_data_dir() -> PathBuf {
    let exe_path = std::env::current_exe().expect("impossible de localiser l'exécutable");
    #[allow(unused_mut)]
    let mut exe_dir = exe_path
        .parent()
        .expect("l'exécutable n'a pas de dossier parent")
        .to_path_buf();

    // Sur macOS, l'exécutable réel vit dans MonApp.app/Contents/MacOS/.
    // Le bundle .app est en lecture seule une fois signé/notarisé : `data/`
    // doit être son voisin (à côté du .app), jamais à l'intérieur.
    #[cfg(target_os = "macos")]
    {
        if let Some(app_bundle) = exe_dir
            .ancestors()
            .find(|p| p.extension().map_or(false, |e| e == "app"))
        {
            if let Some(parent) = app_bundle.parent() {
                exe_dir = parent.to_path_buf();
            }
        }
    }

    exe_dir.join("data")
}

/// Dossier de données de secours, dans le profil utilisateur. Utilisé quand
/// l'emplacement portable n'est pas inscriptible : notamment sur macOS quand
/// l'app tourne depuis un emplacement translocé par Gatekeeper (lancée
/// directement depuis un .dmg ou un .zip fraîchement extrait, sans avoir été
/// déplacée via le Finder au préalable), qui est en lecture seule.
fn fallback_data_dir() -> PathBuf {
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .expect("impossible de localiser le dossier utilisateur");

    #[cfg(target_os = "macos")]
    {
        home.join("Library/Application Support/Cabinet Epidemiologique")
    }
    #[cfg(target_os = "windows")]
    {
        std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .unwrap_or(home)
            .join("Cabinet Epidemiologique")
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        home.join(".local/share/cabinet-epidemiologie")
    }
}

/// Choisit et met en cache le dossier de données réellement utilisé pour
/// cette exécution : le dossier portable s'il est inscriptible, sinon le
/// dossier de secours. Évite qu'un simple souci d'écriture au lancement
/// (ex. translocation macOS) fasse planter l'appli avec un `abort()`.
fn data_dir() -> &'static PathBuf {
    static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();
    DATA_DIR.get_or_init(|| {
        let portable = portable_data_dir();
        if std::fs::create_dir_all(&portable).is_ok() {
            portable
        } else {
            fallback_data_dir()
        }
    })
}

pub fn uploads_dir() -> PathBuf {
    data_dir().join("uploads")
}

pub fn db_path() -> PathBuf {
    data_dir().join("cabinet.sqlite")
}

/// Crée les dossiers de données s'ils n'existent pas encore (premier lancement).
pub fn ensure_dirs() {
    std::fs::create_dir_all(uploads_dir()).expect("création du dossier data/uploads");
}
