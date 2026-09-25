use tauri::State;

use crate::domain::{ReportsBundle, ReportsInput};
use crate::error::AppResult;
use crate::services::ReportsService;
use crate::state::AppState;

#[tauri::command(rename_all = "snake_case")]
pub fn generate_report(
    state: State<'_, AppState>,
    input: ReportsInput,
) -> AppResult<ReportsBundle> {
    let conn = state.conn()?;
    ReportsService::generate(&conn, input)
}