use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use crate::domain::entities::category::Category;
use crate::domain::errors::DomainError;
use crate::domain::repositories::CategoryRepository;

#[derive(Clone)]
pub struct InMemoryCategoryRepository {
    pub categories: Arc<RwLock<HashMap<Uuid, Category>>>,
    pub products_count: Arc<RwLock<HashMap<Uuid, usize>>>
}

impl InMemoryCategoryRepository {
    pub fn new() -> Self {
        Self {
            categories: Arc::new(RwLock::new(HashMap::new())),
            products_count: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn  with_sample_data() -> Self {
        let repo = Self::new();
        let mut categories = HashMap::new();
        let mut products_count = HashMap::new();

        let electronics = Category::new(
            "Electronics".to_string(),
            "Electronic devices and accessories".to_string(),
            None,
        );

        let electronics_id = electronics.id;

        let clothing = Category::new(
            "Clothing".to_string(),
            "Appareil and fashion items".to_string(),
            None,
        );

        let clothing_id = clothing.id;

        let phones = Category::new(
            "Phones".to_string(),
            "Smartphones and mobile devices".to_string(),
            Some(electronics.id),
        );

        let mens_clothing = Category::new(
            "Men's Clothing".to_string(),
            "Clothing for men".to_string(),
            Some(clothing.id),
        );

        categories.insert(electronics.id, electronics);
        categories.insert(clothing.id, clothing);
        categories.insert(phones.id, phones);
        categories.insert(mens_clothing.id, mens_clothing);

        products_count.insert(electronics_id, 10);
        products_count.insert(clothing_id, 5);

        repo.categories.write().unwrap().extend(categories);
        repo.products_count.write().unwrap().extend(products_count);

        repo
    }
}

#[async_trait::async_trait]
impl CategoryRepository for InMemoryCategoryRepository {
    async fn find_all(&self) -> Result<Vec<Category>, DomainError> {
        let categories = self.categories.read().unwrap();
        Ok(categories.values().cloned().collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, DomainError> {
        let categories = self.categories.read().unwrap();
        Ok(categories.get(&id).cloned())
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<Category>, DomainError> {
        let categories = self.categories.read().unwrap();
        Ok(categories.values().find(|c| c.slug == slug).cloned())
    }

    async fn find_children(&self, parent_id: Uuid) -> Result<Vec<Category>, DomainError> {
        let categories = self.categories.read().unwrap();
        let children = categories.values().filter(|c| c.parent_id == Some(parent_id)).cloned().collect();
        Ok(children)
    }

    async fn save(&self, category: Category) -> Result<Category, DomainError> {
        let mut categories = self.categories.write().unwrap();
        categories.insert(category.id, category.clone());
        Ok(category)
    }

    async fn update(&self, category: Category) -> Result<Category, DomainError> {
        let mut categories = self.categories.write().unwrap();

        if !categories.contains_key(&category.id) {
            return Err(DomainError::NotFoundError)
        }

        categories.insert(category.id, category.clone());
        Ok(category)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut categories = self.categories.write().unwrap();
        Ok(categories.remove(&id).is_some())
    }

    async fn has_children(&self, id: Uuid) -> Result<bool, DomainError> {
        let categories = self.categories.read().unwrap();
        Ok(categories.values().any(|c| c.parent_id == Some(id)))
    }

    async fn count_products(&self, id: Uuid) -> Result<usize, DomainError> {
        let counts = self.products_count.read().unwrap();
        Ok(counts.get(&id).cloned().unwrap_or(0))
    }

    async fn get_root_categories(&self) -> Result<Vec<Category>, DomainError> {
        let categories = self.categories.read().unwrap();
        Ok(categories.values().filter(|c| c.is_root_category()).cloned().collect())
    }
}