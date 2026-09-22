use serde::{Deserialize, Serialize};

/// A product category (e.g. "Drinks", "Main dishes").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub sort_order: i32,
}

/// Payload received from the frontend to create a new category.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCategoryInput {
    pub name: String,
}