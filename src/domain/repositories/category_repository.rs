use uuid::Uuid;
use crate::domain::entities::category::Category;
use crate::domain::errors::DomainError;

#[async_trait::async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Category>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, DomainError>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Category>, DomainError>;
    async fn find_children(&self, parent_id: Uuid) -> Result<Vec<Category>, DomainError>;
    async fn save(&self, category: Category) -> Result<Category, DomainError>;
    async fn update(&self, category: Category) -> Result<Category, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<bool, DomainError>;
    async fn has_children(&self, id: Uuid) -> Result<bool, DomainError>;
    async fn count_products(&self, id: Uuid) -> Result<usize, DomainError>;
    async fn get_root_categories(&self) -> Result<Vec<Category>, DomainError>;
}