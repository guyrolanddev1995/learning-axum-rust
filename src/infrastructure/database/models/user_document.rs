use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde::__private228::de::IdentifierDeserializer;
use uuid::Uuid;
use crate::domain::entities::{User, UserRole};
use crate::infrastructure::database::models::uuid_as_string;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(with = "uuid_as_string")]
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserDocument {
    fn from(user: User) -> Self {
        Self {
            object_id: None,
            id: user.id,
            email: user.email,
            password_hash: user.password_hash,
            full_name: user.full_name,
            role: user.role.as_str().to_string(),
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<UserDocument> for User {
    fn from(document: UserDocument) -> Self {
        Self {
            id: document.id,
            email: document.email,
            password_hash: document.password_hash,
            full_name: document.full_name,
            role: UserRole::from_str(&document.role),
            is_active: document.is_active,
            created_at: document.created_at,
            updated_at: document.updated_at,
        }
    }
}