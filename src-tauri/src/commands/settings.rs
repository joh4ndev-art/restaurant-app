use tauri::State;

use crate::error::AppResult;
use crate::services::SettingsService;
use crate::state::AppState;

#[tauri::command(rename_all = "snake_case")]
pub fn get_setting(state: State<'_, AppState>, key: String) -> AppResult<Option<String>> {
    let conn = state.conn()?;
    SettingsService::get(&conn, &key)
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_setting(
    state: State<'_, AppState>,
    key: String,
    value: String,
) -> AppResult<()> {
    let conn = state.conn()?;
    SettingsService::set(&conn, &key, &value)
}