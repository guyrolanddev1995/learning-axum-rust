use uuid::Uuid;
use futures::{TryFutureExt, TryStreamExt};
use mongodb::bson::doc;
use mongodb::{Collection, Database, IndexModel};
use mongodb::options::IndexOptions;
use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use crate::domain::repositories::UserRepository;
use crate::infrastructure::database::models::UserDocument;

#[derive(Clone)]
pub struct MongodbUserRepository {
    collection: Collection<UserDocument>
}

impl MongodbUserRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection::<UserDocument>("users");
        Self { collection }
    }

    pub async fn create_index(&self) -> Result<(), mongodb::error::Error> {
        let id_index = IndexModel::builder()
            .keys(doc! {"id": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build();

        let email_index = IndexModel::builder()
            .keys(doc! {"email": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build();

        self.collection.create_indexes(vec![id_index, email_index])
            .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl UserRepository for MongodbUserRepository {
    async fn find_all(&self) -> Result<Vec<User>, DomainError> {
        let cursor = self.collection.find(doc! {})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let users = cursor.try_collect::<Vec<UserDocument>>()
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?
            .into_iter()
            .map(|doc| User::from(doc))
            .collect();

        Ok(users)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let doc = self.collection.find_one(doc! {"id": id.to_string()})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(doc.map(User::from))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let doc = self.collection.find_one(doc! {"email": email})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(doc.map(User::from))
    }

    async fn save(&self, user: User) -> Result<User, DomainError> {
        let user_doc = UserDocument::from(user.clone());
        self.collection.insert_one(&user_doc)
            .await
            .map_err(|e| {
                if e.to_string().contains("E11000") {
                    DomainError::EmailAlreadyExists
                } else {
                    DomainError::InternalError(e.to_string())
                }
            })?;

        Ok(user)
    }

    async fn update(&self, user: User) -> Result<User, DomainError> {
        let user_doc = UserDocument::from(user.clone());

        let result = self.collection.replace_one(doc! {"id": user.id.to_string()}, &user_doc)
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        if result.matched_count == 0 {
            return Err(DomainError::NotFoundError);
        }

        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DomainError> {
        let result = self.collection.delete_one(doc! {"id": id.to_string()})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(result.deleted_count > 0)
    }

    async fn existing_by_email(&self, email: &str) -> Result<bool, DomainError> {
       let count = self.collection
           .count_documents(doc! {"email": email})
           .await
           .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(count > 0)
    }
}