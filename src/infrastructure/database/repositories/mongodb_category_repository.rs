use mongodb::{Collection, Cursor, Database, IndexModel};
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use uuid::Uuid;
use futures::stream::TryStreamExt;
use crate::domain::entities::category::Category;
use crate::domain::errors::DomainError;
use crate::domain::repositories::CategoryRepository;
use crate::infrastructure::database::models::CategoryDocument;

#[derive(Clone)]
pub struct MongodbCategoryRepository {
    collection: Collection<CategoryDocument>
}

impl MongodbCategoryRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection::<CategoryDocument>("categories");
        Self { collection }
    }

    pub async fn create_indexes(&self) -> Result<(), mongodb::error::Error> {
        let id_index = IndexModel::builder()
            .keys(doc! { "id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        let slug_index = IndexModel::builder()
            .keys(doc! { "slug": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        let parent_index = IndexModel::builder()
            .keys(doc! { "parent_id": 1 })
            .build();

        self.collection.create_indexes(vec![id_index, slug_index, parent_index]).await?;

        Ok(())
    }

    async fn convert_into_category_list(cursor: Cursor<CategoryDocument>) -> Result<Vec<Category>, DomainError> {
       Ok(
           cursor.try_collect::<Vec<CategoryDocument>>()
               .await
               .map_err(|e| DomainError::InternalError(e.to_string()))?
               .into_iter()
               .map(|doc| doc.into())
               .collect()
       )
    }
}

#[async_trait::async_trait]
impl CategoryRepository for MongodbCategoryRepository {
    async fn find_all(&self) -> Result<Vec<Category>, DomainError> {
        let cursor = self.collection
            .find(doc! {})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let categories: Vec<Category> = MongodbCategoryRepository::convert_into_category_list(cursor).await?;
        Ok(categories)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, DomainError> {
        let doc = self.collection
            .find_one(doc! {"id": id.to_string()})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(doc.map(|doc| doc.into()))
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<Category>, DomainError> {
      let doc = self.collection
          .find_one(doc! {"slug": slug})
          .await
          .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(doc.map(|doc| doc.into()))
    }

    async fn find_children(&self, parent_id: Uuid) -> Result<Vec<Category>, DomainError> {
        let cursor = self.collection
            .find(doc! {"parent_id": parent_id.to_string()})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let categories = MongodbCategoryRepository::convert_into_category_list(cursor).await?;
        Ok(categories)
    }

    async fn save(&self, category: Category) -> Result<Category, DomainError> {
        let doc = CategoryDocument::from(category.clone());

        self.collection
            .insert_one(&doc)
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(category)
    }

    async fn update(&self, category: Category) -> Result<Category, DomainError> {
        let doc = CategoryDocument::from(category.clone());

        let result = self.collection
            .replace_one(doc! {"id": category.id.to_string()}, &doc)
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        if result.matched_count == 0 {
            return Err(DomainError::NotFoundError);
        }

        Ok(category)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DomainError> {
       let result = self.collection
           .delete_one(doc! {"id": id.to_string()})
           .await
           .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(result.deleted_count > 0)
    }

    async fn has_children(&self, id: Uuid) -> Result<bool, DomainError> {
        let count = self.collection
            .count_documents(doc! {"parent_id": id.to_string()})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(count > 0)
    }

    async fn count_products(&self, id: Uuid) -> Result<usize, DomainError> {
        Ok(0)
    }

    async fn get_root_categories(&self) -> Result<Vec<Category>, DomainError> {
        let cursor = self.collection
            .find(doc! {"parent_id": null})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let categories = MongodbCategoryRepository::convert_into_category_list(cursor).await?;
        Ok(categories)
    }
}