use rusqlite::{params, Connection};

use crate::domain::{Payment, PaymentMethod};
use crate::error::AppResult;

pub struct PaymentRepository<'a> {
    conn: &'a Connection,
}

impl<'a> PaymentRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn insert(
        &self,
        order_id: i64,
        method: PaymentMethod,
        amount_cents: i64,
    ) -> AppResult<i64> {
        self.conn.execute(
            "INSERT INTO payments (order_id, method, amount_cents)
             VALUES (?1, ?2, ?3)",
            params![order_id, method.as_str(), amount_cents],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn list_for_order(&self, order_id: i64) -> AppResult<Vec<Payment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, order_id, method, amount_cents, created_at
             FROM payments
             WHERE order_id = ?1
             ORDER BY created_at ASC, id ASC",
        )?;

        let rows = stmt.query_map(params![order_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let mut payments = Vec::new();
        for row in rows {
            let r = row?;
            let method = PaymentMethod::from_db(&r.2)
                .map_err(crate::error::AppError::Internal)?;
            payments.push(Payment {
                id: r.0,
                order_id: r.1,
                method,
                amount_cents: r.3,
                created_at: r.4,
            });
        }
        Ok(payments)
    }

    pub fn total_paid(&self, order_id: i64) -> AppResult<i64> {
        let total: i64 = self.conn.query_row(
            "SELECT COALESCE(SUM(amount_cents), 0) FROM payments WHERE order_id = ?1",
            params![order_id],
            |row| row.get(0),
        )?;
        Ok(total)
    }
}