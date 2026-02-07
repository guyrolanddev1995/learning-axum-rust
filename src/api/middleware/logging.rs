use axum::Json;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::time::Instant;
use crate::state::AppState;

pub async fn timing_middleware(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();

    let response = next.run(req).await;
    let duration = start.elapsed();

    println!("{method} {uri} - {:?} - {}", duration, response.status());

    response
}

pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, Response> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    match auth_header {
        Some(token) if token.starts_with("Bearer") => Ok(next.run(req).await),
        _ => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "You not authorized to access this route"
            })),
        )
            .into_response()),
    }
}

pub async fn api_key_middleware(
    State(app_state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let provided_key = req.headers().get("X-API-KEY").and_then(|v| v.to_str().ok());

    match provided_key {
        Some(key) if key == app_state.api_key => Ok(next.run(req).await),
        _ => Err(StatusCode::FORBIDDEN)
    }
}
