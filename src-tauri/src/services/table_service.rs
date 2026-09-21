use rusqlite::Connection;

use crate::domain::{CreateTableInput, Table, TableStatus};
use crate::error::{AppError, AppResult};
use crate::infrastructure::repositories::TableRepository;

/// Business rules for tables.
///
/// The service coordinates the repository and enforces invariants
/// that cannot (or should not) live in the database itself.
pub struct TableService;

impl TableService {
    /// Create a new table after validating the input.
    pub fn create(conn: &Connection, input: CreateTableInput) -> AppResult<Table> {
        let name = input.name.trim().to_string();

        // Rule 1: name must not be empty.
        if name.is_empty() {
            return Err(AppError::Validation(
                "table name cannot be empty".into(),
            ));
        }

        // Rule 2: name must not exceed 50 chars.
        if name.chars().count() > 50 {
            return Err(AppError::Validation(
                "table name cannot exceed 50 characters".into(),
            ));
        }

        // Rule 3: capacity must be within a sane range.
        if input.capacity <= 0 || input.capacity > 50 {
            return Err(AppError::Validation(
                "capacity must be between 1 and 50".into(),
            ));
        }

        // Rule 4: zone, if provided, must not be blank.
        let zone = match input.zone {
            Some(z) => {
                let z = z.trim().to_string();
                if z.is_empty() {
                    None
                } else {
                    Some(z)
                }
            }
            None => None,
        };

        let repo = TableRepository::new(conn);

        // Rule 5: no duplicate names.
        if repo.exists_by_name(&name)? {
            return Err(AppError::Validation(format!(
                "a table named '{name}' already exists"
            )));
        }

        repo.create(&name, zone.as_deref(), input.capacity)
    }

    /// Return every table.
    pub fn list(conn: &Connection) -> AppResult<Vec<Table>> {
        TableRepository::new(conn).list()
    }

    /// Change the status of a table.
    pub fn update_status(
        conn: &Connection,
        id: i64,
        new_status: TableStatus,
    ) -> AppResult<Table> {
        let repo = TableRepository::new(conn);
        let current = repo.get_by_id(id)?;

        // Rule: no-op transitions are rejected to keep the intent explicit.
        if current.status == new_status {
            return Err(AppError::Validation(format!(
                "table is already in status '{}'",
                new_status.as_str()
            )));
        }

        repo.update_status(id, new_status)
    }

    /// Delete a table.
    pub fn delete(conn: &Connection, id: i64) -> AppResult<()> {
        let repo = TableRepository::new(conn);
        // Fails with NotFound if the table doesn't exist.
        repo.get_by_id(id)?;
        repo.delete(id)
    }
}