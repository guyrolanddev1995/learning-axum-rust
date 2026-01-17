use std::sync::Arc;
use uuid::Uuid;
use crate::domain::entities::category::Category;
use crate::domain::errors::DomainError;
use crate::domain::repositories::{CategoryRepository, ProductRepository};

#[derive(Clone)]
pub struct CategoryService {
    pub repository: Arc<dyn CategoryRepository>,
    pub product_repository: Arc<dyn ProductRepository>
}

#[derive(Debug, Clone)]
pub struct CategoryTree {
    pub category: Category,
    pub children: Vec<CategoryTree>
}

impl CategoryService {
    async fn is_descendant(
        &self,
        potential_descendant: Uuid,
        ancestor_id: Uuid
    ) -> Result<bool, DomainError> {
        let mut current_id = potential_descendant;

        loop {
            let category = self.get_category(current_id).await?;

            if let Some(parent_id) = category.parent_id {
                if parent_id == ancestor_id {
                    return Ok(true);
                }
                current_id = parent_id;
            } else {
                return Ok(false);
            }
        }
    }

    fn build_tree_node(
        &self,
        category: Category,
        all_categories: &[Category]
    ) -> Result<CategoryTree, DomainError> {
        let children_categories: Vec<Category> = all_categories
            .iter()
            .filter(|c| c.parent_id == Some(category.id))
            .cloned()
            .collect();

        let mut children = Vec::new();

        for child in children_categories {
            children.push(self.build_tree_node(child, all_categories)?);
        }

        Ok(CategoryTree { category, children })
    }
}

impl CategoryService {
    pub fn new(repository: Arc<dyn CategoryRepository>, product_repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository, product_repository }
    }

    pub async fn list_all_categories(&self) -> Result<Vec<Category>, DomainError> {
        self.repository.find_all().await
    }

    pub async fn list_root_categories(&self) -> Result<Vec<Category>, DomainError> {
        self.repository.get_root_categories().await
    }

    pub async fn get_category(&self, id: Uuid) -> Result<Category, DomainError> {
        self.repository.find_by_id(id).await?.ok_or(DomainError::NotFoundError)
    }

    pub async fn get_category_by_slug(&self, slug: &str) -> Result<Category, DomainError> {
        self.repository.find_by_slug(slug).await?.ok_or(DomainError::NotFoundError)
    }

    pub async fn list_children(&self, parent_id: Uuid) -> Result<Vec<Category>, DomainError> {
        self.get_category(parent_id).await?;
        self.repository.find_children(parent_id).await
    }

    pub async fn create_category(
        &self,
        name: String,
        description: String,
        parent_id: Option<Uuid>
    ) -> Result<Category, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError("Category name cannot be empty".to_string()));
        }

        if let Some(pid) = parent_id {
            let parent = self.get_category(pid).await?;
            if !parent.can_be_parent() {
                return Err(DomainError::ValidationError("Parent category cannot have children".to_string()));
            }
        }

        let slug = Category::generate_slug(&name);

        if self.repository.find_by_slug(&slug).await?.is_some() {
            return Err(DomainError::ValidationError("Category slug already exists".to_string()));
        }

        let category = Category::new(name, description, parent_id);
        self.repository.save(category).await
    }

    pub async fn update_category(&self, id: Uuid, name: Option<String>, description: Option<String>, parent_id: Option<Option<Uuid>>) -> Result<Category, DomainError> {
        let mut category = self.get_category(id).await?;

        if let Some(new_name) = name {
            if new_name.trim().is_empty() {
                return Err(DomainError::ValidationError("Category name cannot be empty".to_string()));
            }

            let new_slug = Category::generate_slug(&new_name);

            if new_slug != category.slug {
                if let Some(_) = self.repository.find_by_slug(&new_slug).await? {
                    return Err(DomainError::ValidationError("Category slug already exists".to_string()));
                }
            }

            category.name = new_name;
            category.slug = new_slug;
        }

        if let Some(new_description) = description {
            category.description = new_description;
        }

        if let Some(new_parent_id) = parent_id {
            if new_parent_id == Some(id) {
                return Err(DomainError::ValidationError("Category cannot be its own parent".to_string()));
            }

            if let Some(pid) = new_parent_id {
                let parent = self.get_category(pid).await?;
                if !parent.can_be_parent() {
                    return Err(DomainError::ValidationError("Parent category cannot have children".to_string()));
                }

                if self.is_descendant(pid, id).await? {
                    return Err(DomainError::ValidationError("Category cannot be a descendant of its own parent".to_string()));
                }
            }

            category.parent_id = new_parent_id;
        }

        self.repository.update(category).await
    }

    pub async fn delete_category(&self, id: Uuid) -> Result<(), DomainError> {
        self.get_category(id).await?;

        if self.repository.has_children(id).await? {
            return Err(DomainError::ValidationError("Category cannot have children".to_string()));
        }

        let product_count = self.repository.count_products(id).await?;
        if product_count > 0 {
            return Err(DomainError::ValidationError(
                format!("Category cannot be deleted because it has {} products", product_count)
            ));
        }

        let deleted = self.repository.delete(id).await?;

        if !deleted {
            return Err(DomainError::NotFoundError);
        }

        Ok(())
    }

    pub async fn activate_category(&self, id: Uuid) -> Result<Category, DomainError> {
        let mut category = self.get_category(id).await?;
        category.activate();
        self.repository.update(category).await
    }

    pub async fn deactivate_category(&self, id: Uuid) -> Result<Category, DomainError> {
        let mut category = self.get_category(id).await?;
        category.deactivate();
        self.repository.update(category).await
    }

    pub async fn get_category_tree(&self) -> Result<Vec<CategoryTree>, DomainError> {
        let all_categories = self.list_all_categories().await?;
        let roots = all_categories.iter()
            .filter(|c| !c.is_root_category())
            .cloned()
            .collect::<Vec<_>>();

        let mut tree = Vec::new();

        for root in roots {
            tree.push(self.build_tree_node(root, &all_categories)?);
        }

        Ok(tree)
    }
}