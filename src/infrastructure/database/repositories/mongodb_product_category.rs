use futures::TryStreamExt;
use mongodb::{Collection, Database, IndexModel};
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use uuid::Uuid;
use crate::domain::entities::product::Product;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ProductRepository;
use crate::infrastructure::database::models::ProductDocument;

#[derive(Clone)]
pub struct MongodbProductCategory {
    collection: Collection<ProductDocument>
}

impl MongodbProductCategory {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection::<ProductDocument>("products");
        Self { collection }
    }

    pub async fn create_indexes(&self) -> Result<(), mongodb::error::Error> {
        let id_index = IndexModel::builder()
            .keys(doc! {"id": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build();

        // Index sur sku (unique)
        let sku_index = IndexModel::builder()
            .keys(doc! {"sku": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build();

        // Index sur category_id
        let category_index = IndexModel::builder()
            .keys(doc! {"category_id": 1})
            .build();

        // Index composé sur category_id + is_active
        let category_active_index = IndexModel::builder()
            .keys(doc! {"category_id": 1, "is_active": 1})
            .build();

        self.collection
            .create_indexes(vec![id_index, sku_index, category_index, category_active_index])
            .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl ProductRepository for MongodbProductCategory {
    async fn find_all(&self) -> Result<Vec<Product>, DomainError> {
        let cursor = self
            .collection
            .find(doc! {})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let products = cursor.try_collect::<Vec<ProductDocument>>()
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?
            .into_iter()
            .map(|doc| Product::from(doc))
            .collect();

        Ok(products)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, DomainError> {
        let doc = self.collection
            .find_one(doc! {"id": id.to_string()})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(doc.map(|d| d.into()))
    }

    async fn find_by_sku(&self, sku: &str) -> Result<Option<Product>, DomainError> {
        let doc = self.collection
            .find_one(doc! {"sku": sku})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(doc.map(|d| d.into()))
    }

    async fn find_by_category(&self, category_id: Uuid) -> Result<Vec<Product>, DomainError> {
        let cursor = self.collection
            .find(doc! {"category_id": category_id.to_string()})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let products = cursor.try_collect::<Vec<ProductDocument>>()
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?
            .into_iter()
            .map(|doc| doc.into())
            .collect();

        Ok(products)
    }

    async fn count_by_category(&self, category_id: Uuid) -> Result<usize, DomainError> {
        let counter = self.collection
            .count_documents(doc! {"category_id": category_id.to_string()})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(counter as usize)
    }

    async fn find_active_by_category(&self, category_id: Uuid) -> Result<Vec<Product>, DomainError> {
        let cursor = self.collection
            .find(doc! {"category_id": category_id.to_string(), "is_active": true})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        let products = cursor.try_collect::<Vec<ProductDocument>>()
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?
            .into_iter()
            .map(|doc| doc.into())
            .collect();

        Ok(products)
    }

    async fn save(&self, product: Product) -> Result<Product, DomainError> {
        let doc = ProductDocument::from(product.clone());
        self.collection
            .insert_one(&doc)
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(product)
    }

    async fn update(&self, product: Product) -> Result<Product, DomainError> {
        let doc = ProductDocument::from(product.clone());

        let result = self.collection
            .replace_one(doc! {"id": product.id.to_string()}, &doc)
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        if result.matched_count == 0 {
            return Err(DomainError::NotFoundError);
        }

        Ok(product)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DomainError> {
        let result = self.collection
            .delete_one(doc! {"id": id.to_string()})
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(result.deleted_count > 0)
    }
}
