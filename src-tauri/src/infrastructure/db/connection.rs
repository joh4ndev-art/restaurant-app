use std::path::Path;

use rusqlite::Connection;

use crate::error::AppResult;

pub fn open(path: &Path) -> AppResult<Connection> {
    let conn = Connection::open(path)?;

    // Pragmas tuned for a local desktop application:
    // - WAL: better read/write concurrency, less locking.
    // - synchronous = NORMAL: safe with WAL, faster than FULL.
    // - foreign_keys = ON: SQLite disables FK by default.
    // - busy_timeout: avoid "database is locked" errors under brief contention.
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA synchronous  = NORMAL;
         PRAGMA foreign_keys = ON;
         PRAGMA busy_timeout = 5000;",
    )?;

    Ok(conn)
}