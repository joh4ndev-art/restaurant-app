mod commands;
mod domain;
mod error;
mod infrastructure;
mod services;
mod state;

use std::fs;

use tauri::Manager;

use crate::infrastructure::db;
use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");

            fs::create_dir_all(&app_dir)?;
            let db_path = app_dir.join("restaurant.db");

            let conn = db::connection::open(&db_path)?;
            db::migrations::run(&conn)?;

            app.manage(AppState::new(conn));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::tables::list_tables,
            commands::tables::create_table,
            commands::tables::update_table_status,
            commands::tables::delete_table,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}