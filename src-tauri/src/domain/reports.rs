use serde::{Deserialize, Serialize};

/// Summary of sales over a period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesSummary {
    pub total_cents: i64,
    pub order_count: i64,
    pub average_ticket_cents: i64,
    pub payment_count: i64,
}

/// A product ranked by units sold in a period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopProduct {
    pub product_id: i64,
    pub product_name: String,
    pub quantity: i64,
    pub revenue_cents: i64,
}

/// Sales aggregated by day within a period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesByDay {
    pub date: String,     // "YYYY-MM-DD"
    pub total_cents: i64,
    pub order_count: i64,
}

/// Full report for a date range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsBundle {
    pub from: String,
    pub to: String,
    pub summary: SalesSummary,
    pub top_products: Vec<TopProduct>,
    pub sales_by_day: Vec<SalesByDay>,
}

/// Input from the frontend: date range in "YYYY-MM-DD" format.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportsInput {
    pub from: String,
    pub to: String,
}