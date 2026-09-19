mod error;
mod infrastructure;
mod state;

use std::fs;

use tauri::Manager;

use crate::infrastructure::db;
use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 1. Resolve a per-user application data directory.
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");

            fs::create_dir_all(&app_dir)?;
            let db_path = app_dir.join("restaurant.db");

            // 2. Open the SQLite connection with the right pragmas.
            let conn = db::connection::open(&db_path)?;

            // 3. Apply pending migrations before anything else uses the DB.
            db::migrations::run(&conn)?;

            // 4. Register shared state for all commands.
            app.manage(AppState::new(conn));

            Ok(())
        })
        // Command handlers will be registered here as we add them.
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}