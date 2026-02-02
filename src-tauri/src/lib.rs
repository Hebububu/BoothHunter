mod booth;
mod commands;
mod database;
mod error;

use tauri::Manager;

use booth::client::BoothClient;
use database::AppDatabase;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(BoothClient::new())
        .invoke_handler(tauri::generate_handler![
            commands::search::search_booth,
            commands::item::get_booth_item,
            commands::db::cache_items,
            commands::db::save_search_history,
            commands::db::get_favorites,
            commands::db::add_favorite,
            commands::db::remove_favorite,
            commands::db::get_popular_avatars,
            commands::db::check_avatars_need_update,
            commands::db::update_popular_avatar,
        ])
        .setup(|app| {
            // Initialize database
            let app_data_dir = app.path().app_data_dir()?;
            let db = AppDatabase::initialize(app_data_dir)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            app.manage(db);

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
