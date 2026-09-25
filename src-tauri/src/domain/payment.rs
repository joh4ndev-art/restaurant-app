use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Cash,
    Card,
    Other,
}

impl PaymentMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentMethod::Cash  => "cash",
            PaymentMethod::Card  => "card",
            PaymentMethod::Other => "other",
        }
    }

    pub fn from_db(value: &str) -> Result<Self, String> {
        match value {
            "cash"          => Ok(PaymentMethod::Cash),
            "card"          => Ok(PaymentMethod::Card),
            "other" | "mixed" => Ok(PaymentMethod::Other),
            other           => Err(format!("unknown payment method: {other}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: i64,
    pub order_id: i64,
    pub method: PaymentMethod,
    pub amount_cents: i64,
    pub created_at: String,
}

/// Snapshot of the payment status of an order at a given moment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSummary {
    pub order_id: i64,
    pub total_cents: i64,
    pub paid_cents: i64,
    pub remaining_cents: i64,
    pub payments: Vec<Payment>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PayOrderInput {
    pub order_id: i64,
    pub method: PaymentMethod,
    pub amount_cents: i64,
}