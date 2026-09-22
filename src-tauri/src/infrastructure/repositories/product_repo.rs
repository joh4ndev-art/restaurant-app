use rusqlite::{params, Connection};

use crate::domain::Product;
use crate::error::{AppError, AppResult};

pub struct ProductRepository<'a> {
    conn: &'a Connection,
}

impl<'a> ProductRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn list(&self) -> AppResult<Vec<Product>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, category_id, name, description, price_cents, active
             FROM products
             ORDER BY name ASC",
        )?;

        let rows = stmt.query_map([], map_row)?;

        let mut products = Vec::new();
        for row in rows {
            products.push(row?);
        }
        Ok(products)
    }

    pub fn get_by_id(&self, id: i64) -> AppResult<Product> {
        self.conn
            .query_row(
                "SELECT id, category_id, name, description, price_cents, active
                 FROM products
                 WHERE id = ?1",
                params![id],
                map_row,
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    AppError::NotFound(format!("product with id {id} not found"))
                }
                other => AppError::Database(other),
            })
    }

    pub fn create(
        &self,
        category_id: i64,
        name: &str,
        description: Option<&str>,
        price_cents: i64,
    ) -> AppResult<Product> {
        self.conn.execute(
            "INSERT INTO products (category_id, name, description, price_cents, active)
             VALUES (?1, ?2, ?3, ?4, 1)",
            params![category_id, name, description, price_cents],
        )?;

        let id = self.conn.last_insert_rowid();
        self.get_by_id(id)
    }

    pub fn update(
        &self,
        id: i64,
        category_id: i64,
        name: &str,
        description: Option<&str>,
        price_cents: i64,
        active: bool,
    ) -> AppResult<Product> {
        let affected = self.conn.execute(
            "UPDATE products
             SET category_id = ?1,
                 name        = ?2,
                 description = ?3,
                 price_cents = ?4,
                 active      = ?5,
                 updated_at  = datetime('now')
             WHERE id = ?6",
            params![category_id, name, description, price_cents, active as i32, id],
        )?;

        if affected == 0 {
            return Err(AppError::NotFound(format!("product with id {id} not found")));
        }
        self.get_by_id(id)
    }

    pub fn delete(&self, id: i64) -> AppResult<()> {
        let affected = self
            .conn
            .execute("DELETE FROM products WHERE id = ?1", params![id])?;

        if affected == 0 {
            return Err(AppError::NotFound(format!("product with id {id} not found")));
        }
        Ok(())
    }

    pub fn exists_by_name_in_category(
        &self,
        name: &str,
        category_id: i64,
        exclude_id: Option<i64>,
    ) -> AppResult<bool> {
        let count: i64 = match exclude_id {
            Some(exclude) => self.conn.query_row(
                "SELECT COUNT(*) FROM products
                 WHERE name = ?1 AND category_id = ?2 AND id != ?3",
                params![name, category_id, exclude],
                |row| row.get(0),
            )?,
            None => self.conn.query_row(
                "SELECT COUNT(*) FROM products
                 WHERE name = ?1 AND category_id = ?2",
                params![name, category_id],
                |row| row.get(0),
            )?,
        };
        Ok(count > 0)
    }
}

/// Map a SQLite row into a `Product`.
/// Extracted to avoid repeating the column-index mapping in every query.
fn map_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Product> {
    let active_int: i32 = row.get(5)?;
    Ok(Product {
        id: row.get(0)?,
        category_id: row.get(1)?,
        name: row.get(2)?,
        description: row.get(3)?,
        price_cents: row.get(4)?,
        active: active_int != 0,
    })
}