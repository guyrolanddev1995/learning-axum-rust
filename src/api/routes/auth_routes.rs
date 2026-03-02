use axum::Router;
use axum::routing::post;
use crate::api::handlers::auth_handler::{login, register};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}