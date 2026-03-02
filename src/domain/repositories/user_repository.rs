use uuid::Uuid;
use crate::domain::entities::User;
use crate::domain::errors::DomainError;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<User>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: User) -> Result<User, DomainError>;
    async fn update(&self, user: User) -> Result<User, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<bool, DomainError>;
    async fn existing_by_email(&self, email: &str) -> Result<bool, DomainError>;
}