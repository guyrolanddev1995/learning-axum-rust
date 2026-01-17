use axum::Router;
use axum::routing::{delete, get, patch, post, put};
use crate::api::handlers::product_handlers;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/products", get(product_handlers::list_products))
        .route("/api/products/{id}", get(product_handlers::get_product))
        .route("/api/products/{id}/with-category", get(product_handlers::get_product_with_category))
        .route("/api/products/{id}", put(product_handlers::update_product))
        .route("/api/products/{id}", delete(product_handlers::delete_product))
        .route("/api/products/sku/{sku}", get(product_handlers::get_products_by_sku))
        .route(
            "/api/products/category/{category_id}",
            get(product_handlers::get_products_by_category),
        )
        .route(
            "/api/products/category/{category_id}/active",
            get(product_handlers::get_active_products_by_category),
        )
        .route("/api/products/{id}/activate", patch(product_handlers::activate_product))
        .route("/api/products/{id}/deactivate", patch(product_handlers::deactivate_product))
        .route("/api/products/{id}/adjust-stock", patch(product_handlers::adjust_stock))
        .route("/api/products", post(product_handlers::create_product))
}