use axum::Router;
use axum::routing::{delete, get, post, put};
use crate::state::AppState;

use crate::api::handlers::category_handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/categories", get(category_handlers::list_root_categories))
        .route("/api/categories/roots", get(category_handlers::list_root_categories))
        .route("/api/categories/tree", get(category_handlers::get_category_tree))

        .route("/api/categories/{id}", get(category_handlers::get_category))
        .route("/api/categories/{id}", put(category_handlers::update_category))
        .route("/api/categories/{id}", delete(category_handlers::delete_category))

        .route("/api/categories/{id}/children", get(category_handlers::get_category_children))
        .route("/api/categories/{id}/activate", post(category_handlers::activate_category))
        .route("/api/categories/{id}/deactivate", post(category_handlers::deactivate_category))

        .route("/api/categories/slug/{slug}", get(category_handlers::get_category_by_slug))

        .route("/api/categories", post(category_handlers::create_category))
}