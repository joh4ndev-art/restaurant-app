use rusqlite::{params, Connection, OptionalExtension};

use crate::error::AppResult;

/// Persistence for the `settings` key/value table.
pub struct SettingsRepository<'a> {
    conn: &'a Connection,
}

impl<'a> SettingsRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Return the value for a key, or `None` if the key does not exist.
    pub fn get(&self, key: &str) -> AppResult<Option<String>> {
        let value = self
            .conn
            .query_row(
                "SELECT value FROM settings WHERE key = ?1",
                params![key],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        Ok(value)
    }

    /// Insert or update a key/value pair.
    pub fn set(&self, key: &str, value: &str) -> AppResult<()> {
        self.conn.execute(
            "INSERT INTO settings (key, value, updated_at)
             VALUES (?1, ?2, datetime('now'))
             ON CONFLICT(key) DO UPDATE SET
                 value      = excluded.value,
                 updated_at = excluded.updated_at",
            params![key, value],
        )?;
        Ok(())
    }
}