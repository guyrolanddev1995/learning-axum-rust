use std::sync::Arc;
use crate::application::services::category_service::CategoryService;
use crate::application::services::product_service::ProductService;
use crate::config::settings::Settings;
use crate::domain::repositories::{CategoryRepository, ProductRepository};
use crate::infrastructure::database::MongoDbConnection;
use crate::infrastructure::database::repositories::{MongodbCategoryRepository, MongodbProductCategory};

#[derive(Clone)]
pub struct AppState {
    pub product_service: ProductService,
    pub category_service: CategoryService,
}

impl AppState {
    pub async fn new(setting: &Settings) -> anyhow::Result<Self> {
        let db_connection = MongoDbConnection::new(&setting.mongodb_url).await?;
        let db = db_connection.database();

        let category_repos = MongodbCategoryRepository::new(db);
        let product_repos = MongodbProductCategory::new(db);

        let (_, _) = tokio::join!(
            category_repos.create_indexes(),
            product_repos.create_indexes()
        );

        let category_repository: Arc<dyn CategoryRepository> = Arc::new(category_repos);
        let product_repository: Arc<dyn ProductRepository> = Arc::new(product_repos);

        let category_service = CategoryService::new(
            Arc::clone(&category_repository),
            Arc::clone(&product_repository)
        );

        let product_service = ProductService::new(Arc::clone(&product_repository), Arc::clone(&category_repository));

        Ok(Self { product_service, category_service })
    }
}