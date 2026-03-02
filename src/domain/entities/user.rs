use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Customer
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Customer => "customer"
        }
    }

    pub fn from_str(role: &str) -> Self {
        match role {
            "admin" => UserRole::Admin,
            _ => UserRole::Customer
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }

    pub fn can_place_order(&self) -> bool {
        self.is_active && self.role != UserRole::Customer
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn deactivate(&mut self) {
        if !self.is_active {
            return;
        }
        
        self.is_active = false;
        self.updated_at = Utc::now();
    }
}