use serde::{Deserialize, Serialize};

/// A sellable product.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub category_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub active: bool,
}

/// Payload received from the frontend to create a new product.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateProductInput {
    pub category_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i64,
}

/// Payload received from the frontend to update a product.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProductInput {
    pub id: i64,
    pub category_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub active: bool,
}