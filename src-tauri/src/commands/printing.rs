use tauri::{AppHandle, State};

use crate::domain::{PrintResult, RestaurantInfo};
use crate::error::AppResult;
use crate::services::receipt_service::{open_path, ReceiptService};
use crate::state::AppState;

#[tauri::command(rename_all = "snake_case")]
pub fn get_restaurant_info(state: State<'_, AppState>) -> AppResult<RestaurantInfo> {
    let conn = state.conn()?;
    ReceiptService::restaurant_info(&conn)
}

#[tauri::command(rename_all = "snake_case")]
pub fn save_restaurant_info(
    state: State<'_, AppState>,
    info: RestaurantInfo,
) -> AppResult<()> {
    let conn = state.conn()?;
    ReceiptService::save_restaurant_info(&conn, &info)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_receipt_folder(app: AppHandle, state: State<'_, AppState>) -> AppResult<String> {
    let conn = state.conn()?;
    let folder = ReceiptService::resolve_output_folder(&app, &conn)?;
    Ok(folder.to_string_lossy().into_owned())
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_receipt_folder(state: State<'_, AppState>, path: String) -> AppResult<()> {
    let conn = state.conn()?;
    ReceiptService::set_output_folder(&conn, &path)
}

#[tauri::command(rename_all = "snake_case")]
pub fn print_receipt(
    app: AppHandle,
    state: State<'_, AppState>,
    order_id: i64,
) -> AppResult<PrintResult> {
    let conn = state.conn()?;
    ReceiptService::generate_receipt(&app, &conn, order_id)
}

#[tauri::command(rename_all = "snake_case")]
pub fn print_kitchen_ticket(
    app: AppHandle,
    state: State<'_, AppState>,
    order_id: i64,
) -> AppResult<PrintResult> {
    let conn = state.conn()?;
    ReceiptService::generate_kitchen_ticket(&app, &conn, order_id)
}

#[tauri::command(rename_all = "snake_case")]
pub fn open_file(path: String) -> AppResult<()> {
    open_path(std::path::Path::new(&path))
}