use rusqlite::Connection;

use crate::domain::{ReportsBundle, ReportsInput};
use crate::error::{AppError, AppResult};
use crate::infrastructure::repositories::ReportsRepository;

/// Maximum number of top products returned.
const TOP_LIMIT: i64 = 20;

pub struct ReportsService;

impl ReportsService {
    pub fn generate(conn: &Connection, input: ReportsInput) -> AppResult<ReportsBundle> {
        validate_date(&input.from, "from")?;
        validate_date(&input.to, "to")?;

        if input.from > input.to {
            return Err(AppError::Validation(
                "'from' date cannot be later than 'to' date".into(),
            ));
        }

        let repo = ReportsRepository::new(conn);
        let summary       = repo.sales_summary(&input.from, &input.to)?;
        let top_products  = repo.top_products(&input.from, &input.to, TOP_LIMIT)?;
        let sales_by_day  = repo.sales_by_day(&input.from, &input.to)?;

        Ok(ReportsBundle {
            from: input.from,
            to: input.to,
            summary,
            top_products,
            sales_by_day,
        })
    }
}

// ---------- helpers ----------

/// Loose validation: expects "YYYY-MM-DD".
fn validate_date(value: &str, field: &str) -> AppResult<()> {
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() != 3
        || parts[0].len() != 4
        || parts[1].len() != 2
        || parts[2].len() != 2
        || !parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
    {
        return Err(AppError::Validation(format!(
            "{field} must be in YYYY-MM-DD format"
        )));
    }
    Ok(())
}