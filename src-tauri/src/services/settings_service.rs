use rusqlite::Connection;

use crate::error::{AppError, AppResult};
use crate::infrastructure::repositories::SettingsRepository;

pub struct SettingsService;

impl SettingsService {
    pub fn get(conn: &Connection, key: &str) -> AppResult<Option<String>> {
        if key.trim().is_empty() {
            return Err(AppError::Validation("setting key cannot be empty".into()));
        }
        SettingsRepository::new(conn).get(key)
    }

    pub fn set(conn: &Connection, key: &str, value: &str) -> AppResult<()> {
        if key.trim().is_empty() {
            return Err(AppError::Validation("setting key cannot be empty".into()));
        }
        SettingsRepository::new(conn).set(key, value)
    }
}