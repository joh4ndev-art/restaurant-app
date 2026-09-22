use tauri::State;

use crate::domain::{Category, CreateCategoryInput};
use crate::error::AppResult;
use crate::services::CategoryService;
use crate::state::AppState;

#[tauri::command(rename_all = "snake_case")]
pub fn list_categories(state: State<'_, AppState>) -> AppResult<Vec<Category>> {
    let conn = state.conn()?;
    CategoryService::list(&conn)
}

#[tauri::command(rename_all = "snake_case")]
pub fn create_category(
    state: State<'_, AppState>,
    input: CreateCategoryInput,
) -> AppResult<Category> {
    let conn = state.conn()?;
    CategoryService::create(&conn, input)
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_category(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let conn = state.conn()?;
    CategoryService::delete(&conn, id)
}