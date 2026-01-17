use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::domain::entities::product::Product;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    pub category_id: Uuid,

    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(length(max = 500))]
    pub description: String,

    #[validate(range(min = 1))]
    pub price: u32,

    pub stock: u32,

    #[validate(length(min = 1, max = 500))]
    pub sku: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProductRequest {
    pub category_id: Option<Uuid>,

    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    #[validate(length(max = 500))]
    pub description: Option<String>,

    #[validate(range(min = 1))]
    pub price: Option<u32>,

    pub stock: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AdjustStockRequest {
    pub quantity: i32
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub category_id: Uuid,
    pub description: String,
    pub price: u32,
    pub stock: u32,
    pub available: bool,
    pub sku: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct ProductWithCategoryResponse {
    #[serde(flatten)]
    pub product: ProductResponse,
    pub category_name: String,
    pub category_slug: String
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id.to_string(),
            category_id: product.category_id,
            name: product.name,
            description: product.description.unwrap_or("".to_string()),
            price: product.price,
            stock: product.stock,
            sku: product.sku,
            available: product.stock > 0,
            is_active: product.is_active,
            created_at: product.created_at.to_rfc3339(),
            updated_at: product.updated_at.to_rfc3339(),
        }
    }
}