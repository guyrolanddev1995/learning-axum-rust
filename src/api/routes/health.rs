use axum::{Json, Router};
use axum::error_handling::HandleErrorLayer;
use axum::routing::{get};
use serde_json::json;
use tower::{Layer, ServiceBuilder};
use crate::api::handlers::app_handlers::app_handler;
use crate::api::middleware::{AuthLayer, LoggingLayer};
use crate::state::AppState;

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new().route("/health", get(health_check))
        .route("/app", get(app_handler))
        .layer(
            ServiceBuilder::new()
                .layer(
                    HandleErrorLayer::new(|err: Box<dyn std::error::Error + Send + Sync>| async move {
                        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": err.to_string() })))
                    })
                )
                .layer(AuthLayer::new("123"))
                .layer(LoggingLayer::new("api"))
        )
}