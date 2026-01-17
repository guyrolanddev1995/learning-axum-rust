use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::application::services::category_service::CategoryTree;
use crate::domain::entities::category::Category;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryDto {
    #[validate(length(min = 1, max = 100 ))]
    pub name: String,

    #[validate(length(min = 10, max = 255))]
    pub description: String,

    pub parent_id: Option<Uuid>
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCategoryDto {
    #[validate(length(min = 1, max = 100 ))]
    pub name: Option<String>,

    #[validate(length(min = 10, max = 255))]
    pub description: Option<String>,


    pub parent_id: Option<Option<Uuid>>
}

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub parent_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String
}

impl From<Category> for CategoryResponse {
    fn from(category: Category) -> Self {
        Self {
            id: category.id,
            name: category.name,
            description: category.description,
            slug: category.slug,
            parent_id: category.parent_id,
            is_active: category.is_active,
            created_at: category.created_at.to_rfc3339(),
            updated_at: category.updated_at.unwrap_or_default().to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CategoryTreeResponse {
    #[serde(flatten)]
    pub category: CategoryResponse,
    pub children: Vec<CategoryTreeResponse>
}

impl From<CategoryTree> for CategoryTreeResponse {
    fn from(tree: CategoryTree) -> Self {
        Self {
            category: CategoryResponse::from(tree.category),
            children: tree.children.into_iter().map(Into::into).collect()
        }
    }
}

