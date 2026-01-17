use std::sync::Arc;
use uuid::Uuid;
use crate::domain::entities::product::Product;
use crate::domain::errors::DomainError;
use crate::domain::repositories::CategoryRepository;
use crate::domain::repositories::ProductRepository;

#[derive(Clone)]
pub struct ProductService {
    product_repository: Arc<dyn ProductRepository>,
    category_repository: Arc<dyn CategoryRepository>,
}

impl ProductService{
    pub fn new(
        product_repository: Arc<dyn ProductRepository>,
        category_repository: Arc<dyn CategoryRepository>,
    ) -> Self {
        Self {
            product_repository,
            category_repository
        }
    }

    pub async fn list_products(&self) -> Result<Vec<Product>, DomainError> {
        self.product_repository.find_all().await
    }

    pub async fn find_product(&self, id: Uuid) -> Result<Product, DomainError> {
        self.product_repository
            .find_by_id(id)
            .await?
            .ok_or(DomainError::NotFoundError)
    }

    pub async fn find_product_by_sku(&self, sku: &str) -> Result<Product, DomainError> {
        self.product_repository.find_by_sku(sku).await?.ok_or(DomainError::NotFoundError)
    }

    pub async fn list_products_by_category(&self, category_id: Uuid) -> Result<Vec<Product>, DomainError> {
        self.category_repository.find_by_id(category_id).await?.ok_or(DomainError::NotFoundError)?;
        self.product_repository.find_by_category(category_id).await
    }

    pub async fn list_active_products_by_category(&self, category_id: Uuid) -> Result<Vec<Product>, DomainError> {
        self.category_repository.find_by_id(category_id).await?.ok_or(DomainError::NotFoundError)?;
        self.product_repository.find_active_by_category(category_id).await
    }

    pub async fn create_product(
        &self,
        name: String,
        category_id: Uuid,
        description: String,
        price: u32,
        stock: u32,
        sku: String,
    ) -> Result<Product, DomainError> {
        if price <= 0 {
            return Err(DomainError::InvalidPriceError);
        }

        if name.trim().is_empty() {
            return Err(DomainError::ValidationError("Product name cannot be empty".to_string()));
        }

        if sku.trim().is_empty() {
            return Err(DomainError::ValidationError("Product SKU cannot be empty".to_string()));
        }

        if self.product_repository.find_by_sku(&sku).await?.is_some() {
            return Err(DomainError::ValidationError("Product SKU already exists".to_string()));
        }

        let category = self.category_repository
            .find_by_id(category_id)
            .await?
            .ok_or(DomainError::ValidationError("Category does not exist".to_string()))?;

        if !category.is_active {
            return Err(DomainError::ValidationError("Category is not active".to_string()));
        }

        let product = Product::new(category_id, name, Some(description), price, stock, sku);
        self.product_repository.save(product).await
    }

    pub async fn update_product(
        &self,
        id: Uuid,
        category_id: Option<Uuid>,
        name: Option<String>,
        description: Option<String>,
        price: Option<u32>,
        stock: Option<u32>,
    ) -> Result<Product, DomainError> {
        let mut product = self.find_product(id).await?;

        if let Some(new_category_id) = category_id {
            let category = self.category_repository.find_by_id(new_category_id).await?.ok_or(DomainError::ValidationError("Category does not exist".to_string()))?;

            if !category.is_active {
                return Err(DomainError::ValidationError("Category is not active".to_string()));
            }

            product.update_category(new_category_id);
        }


        if let Some(name) = name {
            if name.trim().is_empty() { return Err(DomainError::ValidationError("Product name cannot be empty".to_string())); }
            product.update_name(name);
        }

        if let Some(description) = description {
            product.update_description(description);
        }

        if let Some(price) = price {
            if price <= 0 { return Err(DomainError::InvalidPriceError); }
            product.update_price(price);
        }

        if let Some(stock) = stock {
            product.update_stock(stock);
        }

        self.product_repository.update(product).await
    }

    pub async fn delete_product(&self, id: Uuid) -> Result<bool, DomainError> {
        let deleted = self.product_repository.delete(id).await?;
        if !deleted {
            return Err(DomainError::NotFoundError);
        }

        Ok(true)
    }

    pub async fn activate_product(&self, id: Uuid) -> Result<Product, DomainError> {
        let mut product = self.find_product(id).await?;
        product.activate();
        self.product_repository.update(product).await
    }

    pub async fn deactivate_product(&self, id: Uuid) -> Result<Product, DomainError> {
        let mut product = self.find_product(id).await?;
        product.deactivate();
        self.product_repository.update(product).await
    }

    pub async fn adjust_stock(&self, id: Uuid, quantity: i32) -> Result<Product, DomainError> {
        let mut products = self.find_product(id).await?;

        if quantity < 0 {
            let abs_quantity = quantity.abs() as u32;
            products.reduce_stock(abs_quantity).map_err(|e| DomainError::ValidationError(e))?
        } else {
            products.increase_stock(quantity as u32);
        }

        self.product_repository.update(products).await
    }
}