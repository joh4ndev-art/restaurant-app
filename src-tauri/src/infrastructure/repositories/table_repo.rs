use rusqlite::{params, Connection};

use crate::domain::{Table, TableStatus};
use crate::error::{AppError, AppResult};

/// Handles all persistence operations for tables.
/// This is the only place in the codebase that knows the SQL schema
/// for the `tables` table.
pub struct TableRepository<'a> {
    conn: &'a Connection,
}

impl<'a> TableRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Return every table, ordered by name.
    pub fn list(&self) -> AppResult<Vec<Table>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, zone, capacity, status
             FROM tables
             ORDER BY name ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, i32>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let mut tables = Vec::new();
        for row in rows {
            let (id, name, zone, capacity, status_str) = row?;
            let status = TableStatus::from_db(&status_str).map_err(AppError::Internal)?;
            tables.push(Table {
                id,
                name,
                zone,
                capacity,
                status,
            });
        }

        Ok(tables)
    }

    /// Return a single table by id.
    pub fn get_by_id(&self, id: i64) -> AppResult<Table> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, zone, capacity, status
             FROM tables
             WHERE id = ?1",
        )?;

        let row = stmt
            .query_row(params![id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, i32>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    AppError::NotFound(format!("table with id {id} not found"))
                }
                other => AppError::Database(other),
            })?;

        let (id, name, zone, capacity, status_str) = row;
        let status = TableStatus::from_db(&status_str).map_err(AppError::Internal)?;
        Ok(Table {
            id,
            name,
            zone,
            capacity,
            status,
        })
    }

    /// Insert a new table with status `free` and return it.
    pub fn create(&self, name: &str, zone: Option<&str>, capacity: i32) -> AppResult<Table> {
        self.conn.execute(
            "INSERT INTO tables (name, zone, capacity, status)
             VALUES (?1, ?2, ?3, 'free')",
            params![name, zone, capacity],
        )?;

        let id = self.conn.last_insert_rowid();
        self.get_by_id(id)
    }

    /// Update only the status of a table and return the updated row.
    pub fn update_status(&self, id: i64, status: TableStatus) -> AppResult<Table> {
        let affected = self.conn.execute(
            "UPDATE tables SET status = ?1 WHERE id = ?2",
            params![status.as_str(), id],
        )?;

        if affected == 0 {
            return Err(AppError::NotFound(format!("table with id {id} not found")));
        }

        self.get_by_id(id)
    }

    /// Delete a table by id.
    pub fn delete(&self, id: i64) -> AppResult<()> {
        let affected = self
            .conn
            .execute("DELETE FROM tables WHERE id = ?1", params![id])?;

        if affected == 0 {
            return Err(AppError::NotFound(format!("table with id {id} not found")));
        }

        Ok(())
    }

    /// Return true if a table with the given name already exists.
    pub fn exists_by_name(&self, name: &str) -> AppResult<bool> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM tables WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }
}