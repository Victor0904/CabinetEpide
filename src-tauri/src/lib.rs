mod commands;
mod db;
mod models;
mod paths;

use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(db::DbState(Mutex::new(db::init_connection())))
        .invoke_handler(tauri::generate_handler![
            commands::get_all_data,
            commands::add_category,
            commands::rename_category,
            commands::delete_category,
            commands::get_fiche_detail,
            commands::update_fiche,
            commands::upload_fiche,
            commands::delete_fiche,
            commands::upload_media,
            commands::delete_media,
            commands::read_media,
            commands::get_media_path,
            commands::set_parametre,
            commands::export_text_file,
        ])
        .run(tauri::generate_context!())
        .expect("erreur pendant l'exécution de l'application Tauri");
}
