use axum::{Json, Router};
use axum::routing::get;
use serde_json::json;
use crate::state::AppState;

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}