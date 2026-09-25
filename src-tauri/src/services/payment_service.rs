use rusqlite::Connection;

use crate::domain::{
    OrderStatus, PayOrderInput, PaymentSummary, TableStatus,
};
use crate::error::{AppError, AppResult};
use crate::infrastructure::repositories::{
    OrderRepository, PaymentRepository, TableRepository,
};

pub struct PaymentService;

impl PaymentService {
    /// Register a payment for an order.
    ///
    /// Business rules:
    ///   - The order must exist and be in an active state.
    ///   - The amount must be positive.
    ///   - The amount cannot exceed the remaining balance.
    ///   - If the payment covers the total, the order becomes `paid`
    ///     and the table moves to `cleaning`.
    ///
    /// All writes happen inside a transaction.
    pub fn pay(conn: &Connection, input: PayOrderInput) -> AppResult<PaymentSummary> {
        if input.amount_cents <= 0 {
            return Err(AppError::Validation(
                "payment amount must be greater than zero".into(),
            ));
        }

        let order_repo = OrderRepository::new(conn);
        let order = order_repo.get_order(input.order_id)?;

        match order.status {
            OrderStatus::Paid => {
                return Err(AppError::Validation(
                    "order is already paid".into(),
                ));
            }
            OrderStatus::Cancelled => {
                return Err(AppError::Validation(
                    "cannot pay a cancelled order".into(),
                ));
            }
            _ => {}
        }

        let payment_repo = PaymentRepository::new(conn);
        let already_paid = payment_repo.total_paid(input.order_id)?;
        let remaining = order.total_cents - already_paid;

        if remaining <= 0 {
            return Err(AppError::Validation(
                "order has no remaining balance".into(),
            ));
        }

        if input.amount_cents > remaining {
            return Err(AppError::Validation(format!(
                "payment exceeds remaining balance ({} cents)",
                remaining
            )));
        }

        // Persist inside a transaction.
        let tx = conn.unchecked_transaction()?;

        PaymentRepository::new(&tx).insert(
            input.order_id,
            input.method,
            input.amount_cents,
        )?;

        let new_total_paid = already_paid + input.amount_cents;
        let fully_paid = new_total_paid >= order.total_cents;

        if fully_paid {
            OrderRepository::new(&tx)
                .update_order_status(input.order_id, OrderStatus::Paid)?;

            if let Some(table_id) = order.table_id {
                TableRepository::new(&tx)
                    .update_status(table_id, TableStatus::Cleaning)?;
            }
        }

        tx.commit()?;

        Self::summary(conn, input.order_id)
    }

    /// Return the current payment summary for an order.
    pub fn summary(conn: &Connection, order_id: i64) -> AppResult<PaymentSummary> {
        let order = OrderRepository::new(conn).get_order(order_id)?;
        let payment_repo = PaymentRepository::new(conn);
        let payments = payment_repo.list_for_order(order_id)?;
        let paid_cents = payment_repo.total_paid(order_id)?;

        Ok(PaymentSummary {
            order_id,
            total_cents: order.total_cents,
            paid_cents,
            remaining_cents: order.total_cents - paid_cents,
            payments,
        })
    }
}