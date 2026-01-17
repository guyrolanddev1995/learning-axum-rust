use axum::Router;
use crate::config::settings::Settings;
use crate::state::AppState;

pub mod config;
pub mod api;
pub mod state;
pub mod infrastructure;
pub mod application;
pub mod domain;

pub async fn run(setting: Settings) -> anyhow::Result<()> {
    let state = AppState::new(&setting).await?;
    let app = create_app(state);

    let address = format!("{}:{}", setting.host, setting.port);
    let listener = tokio::net::TcpListener::bind(&address).await?;

    tracing::info!("🚀 Server running on http://{}", address);
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_app(state: AppState) -> Router {
  api::routes::create_router(state)
}