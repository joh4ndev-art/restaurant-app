use serde::{Deserialize, Serialize};

/// Information about the restaurant printed on every receipt.
/// All fields are optional except `name`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantInfo {
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub footer_message: Option<String>,
}

impl Default for RestaurantInfo {
    fn default() -> Self {
        Self {
            name: "Restaurant".to_string(),
            address: None,
            phone: None,
            footer_message: None,
        }
    }
}

/// Output format selected by the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrintOutput {
    Pdf,
    // Reserved for future ESC/POS support.
    EscPos,
}

impl PrintOutput {
    pub fn as_str(&self) -> &'static str {
        match self {
            PrintOutput::Pdf    => "pdf",
            PrintOutput::EscPos => "escpos",
        }
    }

    pub fn from_db(value: &str) -> Self {
        match value {
            "escpos" => PrintOutput::EscPos,
            _        => PrintOutput::Pdf,
        }
    }
}

/// Result of generating a receipt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintResult {
    pub path: String,
    pub output: PrintOutput,
}