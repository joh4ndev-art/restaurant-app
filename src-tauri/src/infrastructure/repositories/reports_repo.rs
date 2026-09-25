use rusqlite::{params, Connection};

use crate::domain::{SalesByDay, SalesSummary, TopProduct};
use crate::error::AppResult;

pub struct ReportsRepository<'a> {
    conn: &'a Connection,
}

impl<'a> ReportsRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Aggregate sales totals for payments recorded within [from, to].
    /// `from` and `to` are "YYYY-MM-DD" strings.
    pub fn sales_summary(&self, from: &str, to: &str) -> AppResult<SalesSummary> {
        // Inclusive range: append the day boundary in SQL.
        let total_cents: i64 = self.conn.query_row(
            "SELECT COALESCE(SUM(amount_cents), 0)
             FROM payments
             WHERE date(created_at) BETWEEN date(?1) AND date(?2)",
            params![from, to],
            |row| row.get(0),
        )?;

        let payment_count: i64 = self.conn.query_row(
            "SELECT COUNT(*)
             FROM payments
             WHERE date(created_at) BETWEEN date(?1) AND date(?2)",
            params![from, to],
            |row| row.get(0),
        )?;

        let order_count: i64 = self.conn.query_row(
            "SELECT COUNT(DISTINCT order_id)
             FROM payments
             WHERE date(created_at) BETWEEN date(?1) AND date(?2)",
            params![from, to],
            |row| row.get(0),
        )?;

        let average_ticket_cents = if order_count > 0 {
            total_cents / order_count
        } else {
            0
        };

        Ok(SalesSummary {
            total_cents,
            order_count,
            average_ticket_cents,
            payment_count,
        })
    }

    /// Top products by units sold within the period.
    /// Counts items from orders whose payment was recorded in the period.
    pub fn top_products(&self, from: &str, to: &str, limit: i64) -> AppResult<Vec<TopProduct>> {
        let mut stmt = self.conn.prepare(
            "SELECT
                oi.product_id,
                oi.product_name,
                SUM(oi.quantity)                AS qty,
                SUM(oi.quantity * oi.unit_price_cents) AS revenue
             FROM order_items oi
             JOIN orders o   ON o.id = oi.order_id
             JOIN payments p ON p.order_id = o.id
             WHERE date(p.created_at) BETWEEN date(?1) AND date(?2)
               AND oi.status != 'cancelled'
             GROUP BY oi.product_id, oi.product_name
             ORDER BY qty DESC, revenue DESC
             LIMIT ?3",
        )?;

        let rows = stmt.query_map(params![from, to, limit], |row| {
            Ok(TopProduct {
                product_id: row.get(0)?,
                product_name: row.get(1)?,
                quantity: row.get(2)?,
                revenue_cents: row.get(3)?,
            })
        })?;

        let mut products = Vec::new();
        for row in rows {
            products.push(row?);
        }
        Ok(products)
    }

    /// Sales aggregated by day within the period.
    pub fn sales_by_day(&self, from: &str, to: &str) -> AppResult<Vec<SalesByDay>> {
        let mut stmt = self.conn.prepare(
            "SELECT
                date(p.created_at)         AS day,
                SUM(p.amount_cents)        AS total,
                COUNT(DISTINCT p.order_id) AS orders
             FROM payments p
             WHERE date(p.created_at) BETWEEN date(?1) AND date(?2)
             GROUP BY day
             ORDER BY day ASC",
        )?;

        let rows = stmt.query_map(params![from, to], |row| {
            Ok(SalesByDay {
                date: row.get(0)?,
                total_cents: row.get(1)?,
                order_count: row.get(2)?,
            })
        })?;

        let mut days = Vec::new();
        for row in rows {
            days.push(row?);
        }
        Ok(days)
    }
}