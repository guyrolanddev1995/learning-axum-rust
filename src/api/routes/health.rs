use axum::{Json, Router};
use axum::middleware::{from_fn, from_fn_with_state};
use axum::routing::{get, post};
use serde_json::json;
use tower::ServiceBuilder;
use crate::api::handlers::app_handlers::app_handler;
use crate::api::middleware::{api_key_middleware, auth_middleware, timing_middleware};
use crate::state::AppState;

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new().route("/health", get(health_check))
        .route("/app", post(app_handler))
        .layer(from_fn(timing_middleware))
        .layer(from_fn(auth_middleware))
        .layer(from_fn_with_state(state, api_key_middleware))
}