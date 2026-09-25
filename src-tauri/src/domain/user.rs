use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    Waiter,
    Kitchen,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin   => "admin",
            Role::Waiter  => "waiter",
            Role::Kitchen => "kitchen",
        }
    }

    pub fn from_db(value: &str) -> Result<Self, String> {
        match value {
            "admin"   => Ok(Role::Admin),
            "waiter"  => Ok(Role::Waiter),
            "kitchen" => Ok(Role::Kitchen),
            other     => Err(format!("unknown role: {other}")),
        }
    }
}

/// A user as returned to the frontend. Never exposes pin_hash or pin_salt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub role: Role,
    pub active: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub pin: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub pin: String,
    pub role: Role,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUserInput {
    pub id: i64,
    pub username: String,
    pub role: Role,
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangePinInput {
    pub id: i64,
    pub pin: String,
}