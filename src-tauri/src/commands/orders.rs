use tauri::State;

use crate::domain::{Order, SendOrderInput};
use crate::error::AppResult;
use crate::services::OrderService;
use crate::state::AppState;

#[tauri::command(rename_all = "snake_case")]
pub fn send_order(
    state: State<'_, AppState>,
    input: SendOrderInput,
) -> AppResult<Order> {
    let conn = state.conn()?;
    OrderService::send_to_kitchen(&conn, input)
}

#[tauri::command(rename_all = "snake_case")]
pub fn list_open_orders(state: State<'_, AppState>) -> AppResult<Vec<Order>> {
    let conn = state.conn()?;
    OrderService::list_open(&conn)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_order(state: State<'_, AppState>, id: i64) -> AppResult<Order> {
    let conn = state.conn()?;
    OrderService::get(&conn, id)
}

#[tauri::command(rename_all = "snake_case")]
pub fn cancel_order(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let conn = state.conn()?;
    OrderService::cancel(&conn, id)
}