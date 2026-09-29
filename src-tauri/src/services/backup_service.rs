use std::fs;
use std::path::PathBuf;

use rusqlite::backup::Backup;
use rusqlite::Connection;
use tauri::{AppHandle, Manager};

use crate::domain::BackupInfo;
use crate::error::{AppError, AppResult};
use crate::infrastructure::repositories::SettingsRepository;

const KEY_BACKUP_FOLDER: &str = "backup_folder";
const KEY_AUTO_BACKUP: &str = "auto_backup_enabled";
const KEY_LAST_AUTO: &str = "last_auto_backup_date";

pub struct BackupService;

impl BackupService {
    /// Resolve the folder where backups live. Creates it if missing.
    pub fn resolve_folder(app: &AppHandle, conn: &Connection) -> AppResult<PathBuf> {
        let configured = SettingsRepository::new(conn)
            .get(KEY_BACKUP_FOLDER)?
            .filter(|s| !s.trim().is_empty());

        let folder = match configured {
            Some(path) => PathBuf::from(path),
            None => {
                let app_dir = app
                    .path()
                    .app_data_dir()
                    .map_err(|e| AppError::Internal(format!("no app dir: {e}")))?;
                app_dir.join("backups")
            }
        };

        fs::create_dir_all(&folder)
            .map_err(|e| AppError::Internal(format!("cannot create backup folder: {e}")))?;

        Ok(folder)
    }

    pub fn set_folder(conn: &Connection, path: &str) -> AppResult<()> {
        SettingsRepository::new(conn).set(KEY_BACKUP_FOLDER, path.trim())
    }

    pub fn is_auto_enabled(conn: &Connection) -> AppResult<bool> {
        let value = SettingsRepository::new(conn).get(KEY_AUTO_BACKUP)?;
        Ok(value.as_deref() != Some("false"))
    }

    pub fn set_auto_enabled(conn: &Connection, enabled: bool) -> AppResult<()> {
        SettingsRepository::new(conn).set(KEY_AUTO_BACKUP, if enabled { "true" } else { "false" })
    }

    /// List backups in the folder, newest first.
    pub fn list(app: &AppHandle, conn: &Connection) -> AppResult<Vec<BackupInfo>> {
        let folder = Self::resolve_folder(app, conn)?;
        let mut backups = Vec::new();

        let entries = fs::read_dir(&folder)
            .map_err(|e| AppError::Internal(format!("cannot read backup folder: {e}")))?;

        for entry in entries {
            let entry = entry.map_err(|e| AppError::Internal(e.to_string()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("db") {
                continue;
            }

            let metadata = entry.metadata()
                .map_err(|e| AppError::Internal(e.to_string()))?;

            let secs = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            let created_at = format_timestamp(conn, secs);

            backups.push(BackupInfo {
                filename: path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
                path: path.to_string_lossy().into_owned(),
                size_bytes: metadata.len(),
                created_at,
            });
        }

        // Newest first: filenames include timestamps, so lexical order works.
        backups.sort_by(|a, b| b.filename.cmp(&a.filename));
        Ok(backups)
    }

    /// Create a backup of the current database.
    pub fn create(app: &AppHandle, conn: &Connection) -> AppResult<BackupInfo> {
        let folder = Self::resolve_folder(app, conn)?;
        let now = now_timestamp(conn);
        let slug: String = now
            .chars()
            .map(|c| if c.is_ascii_digit() { c } else { '-' })
            .collect();
        let filename = format!("backup-{slug}.db");
        let path = folder.join(&filename);

        let mut dest = Connection::open(&path)
            .map_err(|e| AppError::Internal(format!("cannot open backup file: {e}")))?;

        {
            let backup = Backup::new(conn, &mut dest)
                .map_err(|e| AppError::Internal(format!("cannot start backup: {e}")))?;

            backup
                .run_to_completion(100, std::time::Duration::from_millis(0), None)
                .map_err(|e| AppError::Internal(format!("backup failed: {e}")))?;
        }

        drop(dest);

        let metadata = fs::metadata(&path)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(BackupInfo {
            filename,
            path: path.to_string_lossy().into_owned(),
            size_bytes: metadata.len(),
            created_at: now,
        })
    }

    /// Restore a backup by filename.
    /// After restoring, the caller is responsible for running migrations.
    pub fn restore(
        app: &AppHandle,
        conn: &mut Connection,
        filename: &str,
    ) -> AppResult<()> {
        validate_filename(filename)?;

        let folder = Self::resolve_folder(app, conn)?;
        let path = folder.join(filename);

        if !path.exists() {
            return Err(AppError::NotFound(format!("backup not found: {filename}")));
        }

        let src = Connection::open(&path)
            .map_err(|e| AppError::Internal(format!("cannot open backup: {e}")))?;

        let backup = Backup::new(&src, conn)
            .map_err(|e| AppError::Internal(format!("cannot start restore: {e}")))?;

        backup
            .run_to_completion(100, std::time::Duration::from_millis(0), None)
            .map_err(|e| AppError::Internal(format!("restore failed: {e}")))?;

        drop(backup);
        drop(src);

        Ok(())
    }

    pub fn delete(app: &AppHandle, conn: &Connection, filename: &str) -> AppResult<()> {
        validate_filename(filename)?;

        let folder = Self::resolve_folder(app, conn)?;
        let path = folder.join(filename);

        if !path.exists() {
            return Err(AppError::NotFound(format!("backup not found: {filename}")));
        }

        fs::remove_file(&path)
            .map_err(|e| AppError::Internal(format!("cannot delete backup: {e}")))?;

        Ok(())
    }

    /// Create a daily auto-backup if today's hasn't been made yet.
    /// Returns Some(info) if a backup was created, None otherwise.
    pub fn run_auto_if_needed(
        app: &AppHandle,
        conn: &Connection,
    ) -> AppResult<Option<BackupInfo>> {
        if !Self::is_auto_enabled(conn)? {
            return Ok(None);
        }

        let today: String = now_timestamp(conn).chars().take(10).collect();
        let last = SettingsRepository::new(conn).get(KEY_LAST_AUTO)?;

        if last.as_deref() == Some(today.as_str()) {
            return Ok(None);
        }

        let info = Self::create(app, conn)?;

        // Record the date so we don't run again today.
        SettingsRepository::new(conn).set(KEY_LAST_AUTO, &today)?;

        Ok(Some(info))
    }
}

// ---------- helpers ----------

fn validate_filename(filename: &str) -> AppResult<()> {
    if filename.is_empty()
        || filename.contains('/')
        || filename.contains('\\')
        || filename.contains("..")
    {
        return Err(AppError::Validation("invalid backup filename".into()));
    }
    Ok(())
}

/// Format a Unix timestamp (seconds) as "YYYY-MM-DD HH:MM:SS" using SQLite.
fn format_timestamp(conn: &Connection, secs: u64) -> String {
    conn.query_row(
        "SELECT strftime('%Y-%m-%d %H:%M:%S', ?1, 'unixepoch', 'localtime')",
        [secs as i64],
        |row| row.get::<_, String>(0),
    )
    .unwrap_or_else(|_| "unknown".into())
}

/// Current local timestamp in "YYYY-MM-DD HH:MM:SS".
fn now_timestamp(conn: &Connection) -> String {
    conn.query_row(
        "SELECT strftime('%Y-%m-%d %H:%M:%S', 'now', 'localtime')",
        [],
        |row| row.get::<_, String>(0),
    )
    .unwrap_or_else(|_| "1970-01-01 00:00:00".into())
}