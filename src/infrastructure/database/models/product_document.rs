use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::product::Product;
use crate::infrastructure::database::models::uuid_as_string;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<ObjectId>,

    #[serde(with = "uuid_as_string")]
    pub id: Uuid,

    pub name: String,

    #[serde(with = "uuid_as_string")]
    pub category_id: Uuid,

    pub description: Option<String>,

    pub price: u32,

    pub stock: u32,

    pub sku: String,

    pub is_active: bool,

    pub created_at: chrono::DateTime<chrono::Utc>,

    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Product> for ProductDocument {
    fn from(product: Product) -> Self {
        Self {
            object_id: None,
            id: product.id,
            name: product.name,
            category_id: product.category_id,
            description: product.description,
            price: product.price,
            stock: product.stock,
            sku: product.sku,
            is_active: product.is_active,
            created_at: product.created_at,
            updated_at: product.updated_at
        }
    }
}

impl From<ProductDocument> for Product {
    fn from(value: ProductDocument) -> Self {
        Self {
            id: value.id,
            name: value.name,
            category_id: value.category_id,
            description: value.description,
            price: value.price,
            stock: value.stock,
            sku: value.sku,
            is_active: value.is_active,
            created_at: value.created_at,
            updated_at: value.updated_at
        }
    }
}