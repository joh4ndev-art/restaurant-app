use tauri::State;

use crate::domain::{CreateProductInput, Product, UpdateProductInput};
use crate::error::AppResult;
use crate::services::ProductService;
use crate::state::AppState;

#[tauri::command(rename_all = "snake_case")]
pub fn list_products(state: State<'_, AppState>) -> AppResult<Vec<Product>> {
    let conn = state.conn()?;
    ProductService::list(&conn)
}

#[tauri::command(rename_all = "snake_case")]
pub fn create_product(
    state: State<'_, AppState>,
    input: CreateProductInput,
) -> AppResult<Product> {
    let conn = state.conn()?;
    ProductService::create(&conn, input)
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_product(
    state: State<'_, AppState>,
    input: UpdateProductInput,
) -> AppResult<Product> {
    let conn = state.conn()?;
    ProductService::update(&conn, input)
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_product(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let conn = state.conn()?;
    ProductService::delete(&conn, id)
}