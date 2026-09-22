use rusqlite::Connection;

use crate::domain::{
    Order, OrderStatus, SendOrderInput, TableStatus,
};
use crate::error::{AppError, AppResult};
use crate::infrastructure::repositories::{
    OrderRepository, ProductRepository, TableRepository,
};

pub struct OrderService;

impl OrderService {
    /// Send an order to the kitchen.
    ///
    /// This is an all-or-nothing operation:
    ///   1. Validate the input.
    ///   2. Create the order row.
    ///   3. Insert every item (with a snapshot of product name + price).
    ///   4. Mark the table as occupied.
    ///
    /// If anything fails, nothing is written.
    pub fn send_to_kitchen(conn: &Connection, input: SendOrderInput) -> AppResult<Order> {
        if input.items.is_empty() {
            return Err(AppError::Validation(
                "order must have at least one item".into(),
            ));
        }

        // Verify the table exists and is available.
        let table_repo = TableRepository::new(conn);
        let table = table_repo.get_by_id(input.table_id)?;
        if table.status != TableStatus::Free {
            return Err(AppError::Validation(format!(
                "table '{}' is not free (current status: {})",
                table.name,
                table.status.as_str()
            )));
        }

        // Validate every item and compute the total.
        let product_repo = ProductRepository::new(conn);
        let mut validated: Vec<(i64, String, i32, i64, Option<String>)> = Vec::new();
        let mut total_cents: i64 = 0;

        for item in &input.items {
            if item.quantity <= 0 {
                return Err(AppError::Validation(
                    "item quantity must be greater than zero".into(),
                ));
            }

            let product = product_repo.get_by_id(item.product_id)?;
            if !product.active {
                return Err(AppError::Validation(format!(
                    "product '{}' is not active",
                    product.name
                )));
            }

            total_cents += product.price_cents * item.quantity as i64;
            validated.push((
                product.id,
                product.name.clone(),
                item.quantity,
                product.price_cents,
                item.notes.clone(),
            ));
        }

        // Persist inside a transaction.
        let tx = conn.unchecked_transaction()?;

        let order_id = {
            let order_repo = OrderRepository::new(&tx);

            let id = order_repo.create_order(
                Some(input.table_id),
                input.notes.as_deref(),
                total_cents,
                OrderStatus::SentToKitchen,
            )?;

            for (product_id, product_name, quantity, unit_price, notes) in &validated {
                order_repo.add_item(
                    id,
                    *product_id,
                    product_name,
                    *quantity,
                    *unit_price,
                    notes.as_deref(),
                )?;
            }
            id
        };

        // Mark table as occupied.
        TableRepository::new(&tx).update_status(input.table_id, TableStatus::Occupied)?;

        tx.commit()?;

        // Return the freshly created order with its items.
        OrderRepository::new(conn).get_order(order_id)
    }

    pub fn list_open(conn: &Connection) -> AppResult<Vec<Order>> {
        OrderRepository::new(conn).list_open_orders()
    }

    pub fn get(conn: &Connection, id: i64) -> AppResult<Order> {
        OrderRepository::new(conn).get_order(id)
    }

    pub fn cancel(conn: &Connection, id: i64) -> AppResult<()> {
        let repo = OrderRepository::new(conn);
        let order = repo.get_order(id)?;

        if matches!(order.status, OrderStatus::Paid | OrderStatus::Cancelled) {
            return Err(AppError::Validation(format!(
                "cannot cancel an order in status '{}'",
                order.status.as_str()
            )));
        }

        let tx = conn.unchecked_transaction()?;

        OrderRepository::new(&tx).update_order_status(id, OrderStatus::Cancelled)?;

        // Free the table by moving it to `cleaning`.
        // This runs in the same transaction: either both succeed, or neither.
        if let Some(table_id) = order.table_id {
            TableRepository::new(&tx).update_status(table_id, TableStatus::Cleaning)?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn get_active_for_table(
        conn: &Connection,
        table_id: i64,
    ) -> AppResult<Option<Order>> {
        OrderRepository::new(conn).get_active_for_table(table_id)
    }

    pub fn mark_served(conn: &Connection, id: i64) -> AppResult<Order> {
        let repo = OrderRepository::new(conn);
        let order = repo.get_order(id)?;

        // Only `sent_to_kitchen` orders can be marked as served.
        if order.status != OrderStatus::SentToKitchen {
            return Err(AppError::Validation(format!(
                "cannot mark as served: order is in status '{}'",
                order.status.as_str()
            )));
        }

        repo.update_order_status(id, OrderStatus::Served)?;
        repo.get_order(id)
    }

    pub fn list_by_statuses(
        conn: &Connection,
        statuses: &[OrderStatus],
    ) -> AppResult<Vec<Order>> {
        OrderRepository::new(conn).list_by_statuses(statuses)
    }
}