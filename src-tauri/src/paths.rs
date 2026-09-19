use std::path::PathBuf;

/// Dossier de données portable : toujours à côté de l'exécutable, jamais dans
/// AppData / Application Support. C'est ce qui permet de glisser l'exécutable
/// + le dossier `data/` dans un .zip et que tout fonctionne au dézippage.
pub fn portable_data_dir() -> PathBuf {
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

pub fn uploads_dir() -> PathBuf {
    portable_data_dir().join("uploads")
}

pub fn db_path() -> PathBuf {
    portable_data_dir().join("cabinet.sqlite")
}

/// Crée les dossiers portables s'ils n'existent pas encore (premier lancement).
pub fn ensure_dirs() {
    std::fs::create_dir_all(uploads_dir()).expect("création du dossier data/uploads");
}
