use axum::Router;
use tower_http::trace::TraceLayer;
use crate::state::AppState;

mod product_routes;
mod health;
mod category_routes;
mod auth_routes;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(health::routes(state.clone()))
        .merge(product_routes::router())
        .merge(category_routes::router())
        .merge(auth_routes::routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}