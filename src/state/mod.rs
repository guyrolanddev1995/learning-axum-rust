use crate::application::services::category_service::CategoryService;
use crate::application::services::product_service::ProductService;
use crate::config::settings::Settings;
use crate::domain::repositories::{CategoryRepository, ProductRepository, UserRepository};
use crate::infrastructure::database::MongoDbConnection;
use crate::infrastructure::database::repositories::{MongodbCategoryRepository, MongodbProductCategory, MongodbUserRepository};
use std::sync::Arc;
use crate::domain::AuthService;

#[derive(Clone)]
pub struct AppState {
    pub product_service: ProductService,
    pub category_service: CategoryService,
    pub auth_service: AuthService,
}

impl AppState {
    pub async fn new(setting: &Settings) -> anyhow::Result<Self> {
        let db_connection = MongoDbConnection::new(&setting.mongodb_url).await?;
        let db = db_connection.database();

        let category_repos = MongodbCategoryRepository::new(db);
        let product_repos = MongodbProductCategory::new(db);
        let user_repos = MongodbUserRepository::new(db);

        let (_, _, _) = tokio::join!(
            category_repos.create_indexes(),
            product_repos.create_indexes(),
            user_repos.create_index()
        );

        let category_repository: Arc<dyn CategoryRepository> = Arc::new(category_repos);
        let product_repository: Arc<dyn ProductRepository> = Arc::new(product_repos);
        let user_repository: Arc<dyn UserRepository> = Arc::new(user_repos);

        let category_service = CategoryService::new(
            Arc::clone(&category_repository),
            Arc::clone(&product_repository),
        );

        let product_service = ProductService::new(
            Arc::clone(&product_repository),
            Arc::clone(&category_repository),
        );

        let auth_service = AuthService::new(
            Arc::clone(&user_repository),
            setting.jwt_secret.clone(),
            setting.jwt_expiration_minutes
        );

        Ok(Self {
            product_service,
            category_service,
            auth_service,
        })
    }
}
