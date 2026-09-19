use std::sync::Mutex;

use rusqlite::Connection;

/// Shared application state, injected into Tauri commands via `State<AppState>`.
pub struct AppState {
    pub db: Mutex<Connection>,
}

impl AppState {
    pub fn new(conn: Connection) -> Self {
        Self {
            db: Mutex::new(conn),
        }
    }
}