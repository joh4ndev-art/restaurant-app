use tauri::State;

use crate::domain::{CreateTableInput, Table, TableStatus};
use crate::error::AppResult;
use crate::services::TableService;
use crate::state::AppState;

/// Return every table in the restaurant.
#[tauri::command(rename_all = "snake_case")]
pub fn list_tables(state: State<'_, AppState>) -> AppResult<Vec<Table>> {
    let conn = state.conn()?;
    TableService::list(&conn)
}

/// Create a new table.
#[tauri::command(rename_all = "snake_case")]
pub fn create_table(
    state: State<'_, AppState>,
    input: CreateTableInput,
) -> AppResult<Table> {
    let conn = state.conn()?;
    TableService::create(&conn, input)
}

/// Change the status of a table.
#[tauri::command(rename_all = "snake_case")]
pub fn update_table_status(
    state: State<'_, AppState>,
    id: i64,
    status: TableStatus,
) -> AppResult<Table> {
    let conn = state.conn()?;
    TableService::update_status(&conn, id, status)
}

/// Delete a table by id.
#[tauri::command(rename_all = "snake_case")]
pub fn delete_table(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let conn = state.conn()?;
    TableService::delete(&conn, id)
}