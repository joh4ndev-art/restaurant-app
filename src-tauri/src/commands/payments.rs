use tauri::State;

use crate::domain::{PayOrderInput, PaymentSummary};
use crate::error::AppResult;
use crate::services::PaymentService;
use crate::state::AppState;

#[tauri::command(rename_all = "snake_case")]
pub fn pay_order(
    state: State<'_, AppState>,
    input: PayOrderInput,
) -> AppResult<PaymentSummary> {
    let conn = state.conn()?;
    PaymentService::pay(&conn, input)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_payment_summary(
    state: State<'_, AppState>,
    order_id: i64,
) -> AppResult<PaymentSummary> {
    let conn = state.conn()?;
    PaymentService::summary(&conn, order_id)
}