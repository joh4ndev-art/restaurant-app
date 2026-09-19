use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("migration error: {0}")]
    Migration(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorPayload<'a> {
    code: &'a str,
    message: String,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (code, message) = match self {
            AppError::Database(e)   => ("database_error",   e.to_string()),
            AppError::Migration(m)  => ("migration_error",  m.clone()),
            AppError::NotFound(m)   => ("not_found",        m.clone()),
            AppError::Validation(m) => ("validation_error", m.clone()),
            AppError::Internal(m)   => ("internal_error",   m.clone()),
        };

        ErrorPayload { code, message }.serialize(serializer)
    }
}

pub type AppResult<T> = Result<T, AppError>;