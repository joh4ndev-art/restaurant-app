use rusqlite::{params, Connection};

use crate::domain::Category;
use crate::error::{AppError, AppResult};

pub struct CategoryRepository<'a> {
    conn: &'a Connection,
}

impl<'a> CategoryRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn list(&self) -> AppResult<Vec<Category>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, sort_order
             FROM categories
             ORDER BY sort_order ASC, name ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                sort_order: row.get(2)?,
            })
        })?;

        let mut categories = Vec::new();
        for row in rows {
            categories.push(row?);
        }
        Ok(categories)
    }

    pub fn get_by_id(&self, id: i64) -> AppResult<Category> {
        self.conn
            .query_row(
                "SELECT id, name, sort_order FROM categories WHERE id = ?1",
                params![id],
                |row| {
                    Ok(Category {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        sort_order: row.get(2)?,
                    })
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    AppError::NotFound(format!("category with id {id} not found"))
                }
                other => AppError::Database(other),
            })
    }

    pub fn create(&self, name: &str) -> AppResult<Category> {
        let next_order: i32 = self.conn.query_row(
            "SELECT COALESCE(MAX(sort_order), -1) + 1 FROM categories",
            [],
            |row| row.get(0),
        )?;

        self.conn.execute(
            "INSERT INTO categories (name, sort_order) VALUES (?1, ?2)",
            params![name, next_order],
        )?;

        let id = self.conn.last_insert_rowid();
        self.get_by_id(id)
    }

    pub fn delete(&self, id: i64) -> AppResult<()> {
        let affected = self
            .conn
            .execute("DELETE FROM categories WHERE id = ?1", params![id])?;

        if affected == 0 {
            return Err(AppError::NotFound(format!(
                "category with id {id} not found"
            )));
        }
        Ok(())
    }

    pub fn exists_by_name(&self, name: &str) -> AppResult<bool> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM categories WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn count_products(&self, id: i64) -> AppResult<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM products WHERE category_id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        Ok(count)
    }
}