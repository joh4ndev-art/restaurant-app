use std::sync::{Mutex, MutexGuard};

use rusqlite::Connection;

use crate::error::{AppError, AppResult};

/// Shared application state, injected into Tauri commands via `State<AppState>`.
pub struct AppState {
    db: Mutex<Connection>,
}

impl AppState {
    pub fn new(conn: Connection) -> Self {
        Self {
            db: Mutex::new(conn),
        }
    }

    /// Acquire the database lock and return a guard to the connection.
    ///
    /// Returns `AppError::Internal` if the mutex has been poisoned
    /// (which only happens if another thread panicked while holding it).
    pub fn conn(&self) -> AppResult<MutexGuard<'_, Connection>> {
        self.db
            .lock()
            .map_err(|_| AppError::Internal("database mutex poisoned".into()))
    }
}