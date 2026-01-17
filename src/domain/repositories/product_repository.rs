use uuid::Uuid;
use crate::domain::entities::product::Product;
use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Product>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, DomainError>;
    async fn find_by_sku(&self, sku: &str) -> Result<Option<Product>, DomainError>;
    async fn find_by_category(&self, category_id: Uuid) -> Result<Vec<Product>, DomainError>;
    async fn count_by_category(&self, category_id: Uuid) -> Result<usize, DomainError>;
    async fn find_active_by_category(&self, category_id: Uuid) -> Result<Vec<Product>, DomainError>;
    async fn save(&self, product: Product) -> Result<Product, DomainError>;
    async fn update(&self, product: Product) -> Result<Product, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<bool, DomainError>;
}