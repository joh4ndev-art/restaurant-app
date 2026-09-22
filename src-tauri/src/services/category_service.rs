use rusqlite::Connection;

use crate::domain::{Category, CreateCategoryInput};
use crate::error::{AppError, AppResult};
use crate::infrastructure::repositories::CategoryRepository;

pub struct CategoryService;

impl CategoryService {
    pub fn list(conn: &Connection) -> AppResult<Vec<Category>> {
        CategoryRepository::new(conn).list()
    }

    pub fn create(conn: &Connection, input: CreateCategoryInput) -> AppResult<Category> {
        let name = input.name.trim().to_string();

        if name.is_empty() {
            return Err(AppError::Validation("category name cannot be empty".into()));
        }
        if name.chars().count() > 50 {
            return Err(AppError::Validation(
                "category name cannot exceed 50 characters".into(),
            ));
        }

        let repo = CategoryRepository::new(conn);
        if repo.exists_by_name(&name)? {
            return Err(AppError::Validation(format!(
                "a category named '{name}' already exists"
            )));
        }

        repo.create(&name)
    }

    pub fn delete(conn: &Connection, id: i64) -> AppResult<()> {
        let repo = CategoryRepository::new(conn);

        // Verify existence first for a clear error message.
        repo.get_by_id(id)?;

        // Refuse to delete a category that still has products.
        let product_count = repo.count_products(id)?;
        if product_count > 0 {
            return Err(AppError::Validation(format!(
                "cannot delete category: it still has {product_count} product(s)"
            )));
        }

        repo.delete(id)
    }
}