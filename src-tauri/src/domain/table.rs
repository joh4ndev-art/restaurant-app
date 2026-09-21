use serde::{Deserialize, Serialize};

/// Possible states for a table in the restaurant floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TableStatus {
    Free,
    Occupied,
    Reserved,
    Cleaning,
}

impl TableStatus {
    /// Convert a status into the exact string stored in SQLite.
    /// Kept in sync with the CHECK constraint in `001_initial.sql`.
    pub fn as_str(&self) -> &'static str {
        match self {
            TableStatus::Free     => "free",
            TableStatus::Occupied => "occupied",
            TableStatus::Reserved => "reserved",
            TableStatus::Cleaning => "cleaning",
        }
    }

    /// Parse a status coming from SQLite.
    /// Returns an error string if the value is unknown.
    pub fn from_db(value: &str) -> Result<Self, String> {
        match value {
            "free"     => Ok(TableStatus::Free),
            "occupied" => Ok(TableStatus::Occupied),
            "reserved" => Ok(TableStatus::Reserved),
            "cleaning" => Ok(TableStatus::Cleaning),
            other      => Err(format!("unknown table status: {other}")),
        }
    }
}

/// A table on the restaurant floor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: i64,
    pub name: String,
    pub zone: Option<String>,
    pub capacity: i32,
    pub status: TableStatus,
}

/// Payload received from the frontend to create a new table.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTableInput {
    pub name: String,
    pub zone: Option<String>,
    pub capacity: i32,
}