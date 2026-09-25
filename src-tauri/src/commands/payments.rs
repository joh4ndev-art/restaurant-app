use tauri::{AppHandle, Emitter, State};

use crate::domain::{PayOrderInput, PaymentSummary};
use crate::error::AppResult;
use crate::services::PaymentService;
use crate::state::AppState;

const EVENTS_ORDERS_CHANGED: &str = "orders:changed";

#[tauri::command(rename_all = "snake_case")]
pub fn pay_order(
    app: AppHandle,
    state: State<'_, AppState>,
    input: PayOrderInput,
) -> AppResult<PaymentSummary> {
    let conn = state.conn()?;
    let summary = PaymentService::pay(&conn, input)?;
    let _ = app.emit(EVENTS_ORDERS_CHANGED, ());
    Ok(summary)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_payment_summary(
    state: State<'_, AppState>,
    order_id: i64,
) -> AppResult<PaymentSummary> {
    let conn = state.conn()?;
    PaymentService::summary(&conn, order_id)
}