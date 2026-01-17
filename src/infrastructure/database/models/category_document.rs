use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::entities::category::Category;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<ObjectId>,
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub parent_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>
}

impl From<Category> for CategoryDocument {
    fn from(value: Category) -> Self {
        Self {
            object_id: None,
            id: value.id,
            name: value.name,
            description: value.description,
            slug: value.slug,
            parent_id: value.parent_id,
            is_active: value.is_active,
            created_at: chrono::Utc::now(),
            updated_at: Some(chrono::Utc::now())
        }
    }
}

impl From<CategoryDocument> for Category {
    fn from(value: CategoryDocument) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            slug: value.slug,
            parent_id: value.parent_id,
            is_active: value.is_active,
            created_at: value.created_at,
            updated_at: value.updated_at
        }
    }
}