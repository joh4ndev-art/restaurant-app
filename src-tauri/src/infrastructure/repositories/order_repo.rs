use rusqlite::{params, Connection};

use crate::domain::{
    Order, OrderItem, OrderItemStatus, OrderStatus,
};
use crate::error::{AppError, AppResult};

pub struct OrderRepository<'a> {
    conn: &'a Connection,
}

impl<'a> OrderRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create_order(
        &self,
        table_id: Option<i64>,
        notes: Option<&str>,
        total_cents: i64,
        status: OrderStatus,
    ) -> AppResult<i64> {
        self.conn.execute(
            "INSERT INTO orders (table_id, status, total_cents, notes)
             VALUES (?1, ?2, ?3, ?4)",
            params![table_id, status.as_str(), total_cents, notes],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn add_item(
        &self,
        order_id: i64,
        product_id: i64,
        product_name: &str,
        quantity: i32,
        unit_price_cents: i64,
        notes: Option<&str>,
    ) -> AppResult<i64> {
        self.conn.execute(
            "INSERT INTO order_items
                (order_id, product_id, product_name, quantity, unit_price_cents, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                order_id,
                product_id,
                product_name,
                quantity,
                unit_price_cents,
                notes
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_order(&self, id: i64) -> AppResult<Order> {
        let mut stmt = self.conn.prepare(
            "SELECT id, table_id, status, total_cents, notes, created_at, closed_at
             FROM orders WHERE id = ?1",
        )?;

        let order = stmt
            .query_row(params![id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, Option<i64>>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, Option<String>>(6)?,
                ))
            })
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    AppError::NotFound(format!("order with id {id} not found"))
                }
                other => AppError::Database(other),
            })?;

        let status = OrderStatus::from_db(&order.2).map_err(AppError::Internal)?;
        let items = self.list_items(id)?;

        Ok(Order {
            id: order.0,
            table_id: order.1,
            status,
            total_cents: order.3,
            notes: order.4,
            created_at: order.5,
            closed_at: order.6,
            items,
        })
    }

    pub fn list_items(&self, order_id: i64) -> AppResult<Vec<OrderItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, order_id, product_id, product_name, quantity,
                    unit_price_cents, notes, status
             FROM order_items WHERE order_id = ?1
             ORDER BY id ASC",
        )?;

        let rows = stmt.query_map(params![order_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i32>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, String>(7)?,
            ))
        })?;

        let mut items = Vec::new();
        for row in rows {
            let r = row?;
            let status = OrderItemStatus::from_db(&r.7).map_err(AppError::Internal)?;
            items.push(OrderItem {
                id: r.0,
                order_id: r.1,
                product_id: r.2,
                product_name: r.3,
                quantity: r.4,
                unit_price_cents: r.5,
                notes: r.6,
                status,
            });
        }
        Ok(items)
    }

    pub fn list_open_orders(&self) -> AppResult<Vec<Order>> {
        let mut stmt = self.conn.prepare(
            "SELECT id FROM orders
             WHERE status IN ('open', 'sent_to_kitchen', 'served')
             ORDER BY created_at DESC",
        )?;

        let ids: Vec<i64> = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<_, _>>()?;

        let mut orders = Vec::new();
        for id in ids {
            orders.push(self.get_order(id)?);
        }
        Ok(orders)
    }

    pub fn update_order_status(&self, id: i64, status: OrderStatus) -> AppResult<()> {
        let closed_at_sql = match status {
            OrderStatus::Paid | OrderStatus::Cancelled => "datetime('now')",
            _ => "NULL",
        };

        let sql = format!(
            "UPDATE orders SET status = ?1, closed_at = {closed_at_sql} WHERE id = ?2"
        );

        let affected = self.conn.execute(&sql, params![status.as_str(), id])?;
        if affected == 0 {
            return Err(AppError::NotFound(format!("order with id {id} not found")));
        }
        Ok(())
    }
}