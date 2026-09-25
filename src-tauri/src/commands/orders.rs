use tauri::{AppHandle, Emitter, State};

use crate::domain::{Order, SendOrderInput};
use crate::error::AppResult;
use crate::services::OrderService;
use crate::state::AppState;

/// Event name emitted whenever the set of orders changes.
/// Frontend listeners subscribe to this to refresh their views.
const EVENTS_ORDERS_CHANGED: &str = "orders:changed";

#[tauri::command(rename_all = "snake_case")]
pub fn send_order(
    app: AppHandle,
    state: State<'_, AppState>,
    input: SendOrderInput,
) -> AppResult<Order> {
    let conn = state.conn()?;
    let order = OrderService::send_to_kitchen(&conn, input)?;
    let _ = app.emit(EVENTS_ORDERS_CHANGED, ());
    Ok(order)
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
pub fn cancel_order(
    app: AppHandle,
    state: State<'_, AppState>,
    id: i64,
) -> AppResult<()> {
    let conn = state.conn()?;
    OrderService::cancel(&conn, id)?;
    let _ = app.emit(EVENTS_ORDERS_CHANGED, ());
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_active_order_for_table(
    state: State<'_, AppState>,
    table_id: i64,
) -> AppResult<Option<Order>> {
    let conn = state.conn()?;
    OrderService::get_active_for_table(&conn, table_id)
}

#[tauri::command(rename_all = "snake_case")]
pub fn mark_order_served(
    app: AppHandle,
    state: State<'_, AppState>,
    id: i64,
) -> AppResult<Order> {
    let conn = state.conn()?;
    let order = OrderService::mark_served(&conn, id)?;
    let _ = app.emit(EVENTS_ORDERS_CHANGED, ());
    Ok(order)
}

#[tauri::command(rename_all = "snake_case")]
pub fn list_orders_by_status(
    state: State<'_, AppState>,
    statuses: Vec<crate::domain::OrderStatus>,
) -> AppResult<Vec<Order>> {
    let conn = state.conn()?;
    OrderService::list_by_statuses(&conn, &statuses)
}