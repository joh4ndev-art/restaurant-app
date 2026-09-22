use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Open,
    SentToKitchen,
    Served,
    Paid,
    Cancelled,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Open           => "open",
            OrderStatus::SentToKitchen  => "sent_to_kitchen",
            OrderStatus::Served         => "served",
            OrderStatus::Paid           => "paid",
            OrderStatus::Cancelled      => "cancelled",
        }
    }

    pub fn from_db(value: &str) -> Result<Self, String> {
        match value {
            "open"             => Ok(OrderStatus::Open),
            "sent_to_kitchen"  => Ok(OrderStatus::SentToKitchen),
            "served"           => Ok(OrderStatus::Served),
            "paid"             => Ok(OrderStatus::Paid),
            "cancelled"        => Ok(OrderStatus::Cancelled),
            other              => Err(format!("unknown order status: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderItemStatus {
    Pending,
    Preparing,
    Ready,
    Served,
    Cancelled,
}

impl OrderItemStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderItemStatus::Pending    => "pending",
            OrderItemStatus::Preparing  => "preparing",
            OrderItemStatus::Ready      => "ready",
            OrderItemStatus::Served     => "served",
            OrderItemStatus::Cancelled  => "cancelled",
        }
    }

    pub fn from_db(value: &str) -> Result<Self, String> {
        match value {
            "pending"    => Ok(OrderItemStatus::Pending),
            "preparing"  => Ok(OrderItemStatus::Preparing),
            "ready"      => Ok(OrderItemStatus::Ready),
            "served"     => Ok(OrderItemStatus::Served),
            "cancelled"  => Ok(OrderItemStatus::Cancelled),
            other        => Err(format!("unknown order item status: {other}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: i64,
    pub table_id: Option<i64>,
    pub status: OrderStatus,
    pub total_cents: i64,
    pub notes: Option<String>,
    pub created_at: String,
    pub closed_at: Option<String>,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: i64,
    pub order_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub quantity: i32,
    pub unit_price_cents: i64,
    pub notes: Option<String>,
    pub status: OrderItemStatus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendOrderItemInput {
    pub product_id: i64,
    pub quantity: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendOrderInput {
    pub table_id: i64,
    pub notes: Option<String>,
    pub items: Vec<SendOrderItemInput>,
}