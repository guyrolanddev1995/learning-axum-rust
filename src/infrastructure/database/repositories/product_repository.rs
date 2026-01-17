use crate::domain::entities::product::Product;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ProductRepository;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use async_trait::async_trait;

#[derive(Clone)]
pub struct InMemoryProductRepository {
    products: Arc<RwLock<HashMap<Uuid, Product>>>,
}

impl InMemoryProductRepository {
    pub fn new() -> Self {
        Self {
            products: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn with_products_seeder(category_ids: Vec<Uuid>) -> Self {
        let repos = Self::new();
        let mut products = HashMap::new();
        let _now = chrono::Utc::now();

        if !category_ids.is_empty() {
            let laptop = Product::new(
                category_ids[0],
                "Gaming Laptop".to_string(),
                Some("High-performance gaming laptop with RTX 4080".to_string()),
                1000,
                10,
                "laptop-sku".to_string()
            );
            let phone = Product::new(
                category_ids[0],
                "Smartphone Pro".to_string(),
                Some("Latest flagship smartphone".to_string()),
                500,
                5,
                "PHONE-001".to_string()
            );
            let tshirt = Product::new(
                category_ids[1],
                "T-Shirt".to_string(),
                Some("Black T-Shirt".to_string()),
                10,
                100,
                "TSHIRT-001".to_string()
            );

            products.insert(laptop.id, laptop);
            products.insert(phone.id, phone);
            products.insert(tshirt.id, tshirt);
        }

        repos.products.write().unwrap().extend(products);
        repos
    }
}

#[async_trait]
impl ProductRepository for InMemoryProductRepository {
    async fn find_all(&self) -> Result<Vec<Product>, DomainError> {
        let products = self.products.read().unwrap();
        Ok(products.values().cloned().collect::<Vec<Product>>())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, DomainError> {
        let products = self.products.read().unwrap();
        Ok(products.get(&id).cloned())
    }

    async fn find_by_sku(&self, sku: &str) -> Result<Option<Product>, DomainError> {
        let products = self.products.read().unwrap();
        Ok(products.values().find(|p| p.sku == sku.to_string()).cloned())
    }

    async fn find_by_category(&self, category_id: Uuid) -> Result<Vec<Product>, DomainError> {
        let products = self.products.read().unwrap();
        let filtered = products.values().filter(|p| p.category_id == category_id).cloned().collect::<Vec<Product>>();
        Ok(filtered)
    }

    async fn count_by_category(&self, category_id: Uuid) -> Result<usize, DomainError> {
        let products = self.products.read().unwrap();
        Ok(products.values().filter(|p| p.category_id == category_id).count())
    }

    async fn find_active_by_category(&self, category_id: Uuid) -> Result<Vec<Product>, DomainError> {
        let products = self.products.read().unwrap();
        Ok(products.values().filter(|p| p.category_id == category_id && p.is_active).cloned().collect::<Vec<Product>>())
    }

    async fn save(&self, product: Product) -> Result<Product, DomainError> {
        let mut products = self.products.write().unwrap();
        products.insert(product.id, product.clone());
        Ok(product)
    }

    async fn update(&self, product: Product) -> Result<Product, DomainError> {
        let mut products = self.products.write().unwrap();
        products.insert(product.id, product.clone());
        Ok(product)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut products = self.products.write().unwrap();
        Ok(products.remove(&id).is_some())
    }
}
