use rusqlite::{params, Connection};

use crate::error::{AppError, AppResult};

struct Migration {
    version: i32,
    name: &'static str,
    sql: &'static str,
}

/// All migrations, in ascending order by version.
/// To add a new one: create a new `.sql` file next to `001_initial.sql`
/// and append a `Migration` entry here. Never modify an already-applied migration.
const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "initial_schema",
        sql: include_str!("migrations/001_initial.sql"),
    },
    Migration {
        version: 2,
        name: "users_and_roles",
        sql: include_str!("migrations/002_users.sql"),
    },
];

pub fn run(conn: &Connection) -> AppResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version    INTEGER PRIMARY KEY,
            name       TEXT    NOT NULL,
            applied_at TEXT    NOT NULL DEFAULT (datetime('now'))
        );",
    )?;

    let current: i32 = conn.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
        [],
        |row| row.get(0),
    )?;

    for migration in MIGRATIONS {
        if migration.version <= current {
            continue;
        }

        conn.execute_batch(migration.sql).map_err(|e| {
            AppError::Migration(format!(
                "failed to apply migration {} ({}): {}",
                migration.version, migration.name, e
            ))
        })?;

        conn.execute(
            "INSERT INTO schema_migrations (version, name) VALUES (?1, ?2)",
            params![migration.version, migration.name],
        )?;
    }

    Ok(())
}