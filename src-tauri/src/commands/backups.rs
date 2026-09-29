use tauri::{AppHandle, State};

use crate::domain::BackupInfo;
use crate::error::AppResult;
use crate::services::BackupService;
use crate::state::AppState;

#[tauri::command(rename_all = "snake_case")]
pub fn list_backups(
    app: AppHandle,
    state: State<'_, AppState>,
) -> AppResult<Vec<BackupInfo>> {
    let conn = state.conn()?;
    BackupService::list(&app, &conn)
}

#[tauri::command(rename_all = "snake_case")]
pub fn create_backup(
    app: AppHandle,
    state: State<'_, AppState>,
) -> AppResult<BackupInfo> {
    let conn = state.conn()?;
    BackupService::create(&app, &conn)
}

#[tauri::command(rename_all = "snake_case")]
pub fn restore_backup(
    app: AppHandle,
    state: State<'_, AppState>,
    filename: String,
) -> AppResult<()> {
    let mut guard = state.conn()?;
    BackupService::restore(&app, &mut guard, &filename)?;
    // Ensure the schema is up to date after restoring an older backup.
    crate::infrastructure::db::migrations::run(&guard)?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_backup(
    app: AppHandle,
    state: State<'_, AppState>,
    filename: String,
) -> AppResult<()> {
    let conn = state.conn()?;
    BackupService::delete(&app, &conn, &filename)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_backup_folder(
    app: AppHandle,
    state: State<'_, AppState>,
) -> AppResult<String> {
    let conn = state.conn()?;
    let folder = BackupService::resolve_folder(&app, &conn)?;
    Ok(folder.to_string_lossy().into_owned())
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_backup_folder(state: State<'_, AppState>, path: String) -> AppResult<()> {
    let conn = state.conn()?;
    BackupService::set_folder(&conn, &path)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_auto_backup(state: State<'_, AppState>) -> AppResult<bool> {
    let conn = state.conn()?;
    BackupService::is_auto_enabled(&conn)
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_auto_backup(state: State<'_, AppState>, enabled: bool) -> AppResult<()> {
    let conn = state.conn()?;
    BackupService::set_auto_enabled(&conn, enabled)
}