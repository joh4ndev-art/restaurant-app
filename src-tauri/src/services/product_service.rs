use rusqlite::Connection;

use crate::domain::{CreateProductInput, Product, UpdateProductInput};
use crate::error::{AppError, AppResult};
use crate::infrastructure::repositories::{CategoryRepository, ProductRepository};

/// Maximum allowed price in cents (100,000.00 in display currency).
const MAX_PRICE_CENTS: i64 = 10_000_000;

pub struct ProductService;

impl ProductService {
    pub fn list(conn: &Connection) -> AppResult<Vec<Product>> {
        ProductRepository::new(conn).list()
    }

    pub fn create(conn: &Connection, input: CreateProductInput) -> AppResult<Product> {
        let name = input.name.trim().to_string();
        validate_name(&name)?;
        validate_price(input.price_cents)?;

        let category_repo = CategoryRepository::new(conn);
        category_repo.get_by_id(input.category_id)?;

        let product_repo = ProductRepository::new(conn);
        if product_repo.exists_by_name_in_category(&name, input.category_id, None)? {
            return Err(AppError::Validation(format!(
                "a product named '{name}' already exists in this category"
            )));
        }

        let description = normalize_optional(input.description);
        product_repo.create(
            input.category_id,
            &name,
            description.as_deref(),
            input.price_cents,
        )
    }

    pub fn update(conn: &Connection, input: UpdateProductInput) -> AppResult<Product> {
        let name = input.name.trim().to_string();
        validate_name(&name)?;
        validate_price(input.price_cents)?;

        let category_repo = CategoryRepository::new(conn);
        category_repo.get_by_id(input.category_id)?;

        let product_repo = ProductRepository::new(conn);
        product_repo.get_by_id(input.id)?; // fails if not found

        if product_repo.exists_by_name_in_category(&name, input.category_id, Some(input.id))? {
            return Err(AppError::Validation(format!(
                "a product named '{name}' already exists in this category"
            )));
        }

        let description = normalize_optional(input.description);
        product_repo.update(
            input.id,
            input.category_id,
            &name,
            description.as_deref(),
            input.price_cents,
            input.active,
        )
    }

    pub fn delete(conn: &Connection, id: i64) -> AppResult<()> {
        let repo = ProductRepository::new(conn);
        repo.get_by_id(id)?; // fails if not found
        repo.delete(id)
    }
}

// ---------- helpers ----------

fn validate_name(name: &str) -> AppResult<()> {
    if name.is_empty() {
        return Err(AppError::Validation("product name cannot be empty".into()));
    }
    if name.chars().count() > 100 {
        return Err(AppError::Validation(
            "product name cannot exceed 100 characters".into(),
        ));
    }
    Ok(())
}

fn validate_price(price_cents: i64) -> AppResult<()> {
    if price_cents <= 0 {
        return Err(AppError::Validation("price must be greater than zero".into()));
    }
    if price_cents > MAX_PRICE_CENTS {
        return Err(AppError::Validation(format!(
            "price cannot exceed {MAX_PRICE_CENTS} cents"
        )));
    }
    Ok(())
}

fn normalize_optional(value: Option<String>) -> Option<String> {
    value.and_then(|s| {
        let trimmed = s.trim().to_string();
        if trimmed.is_empty() { None } else { Some(trimmed) }
    })
}