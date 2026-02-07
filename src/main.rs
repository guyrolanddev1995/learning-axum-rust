use axumApp::config::settings::Settings;
use axumApp::run;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug")),
        )
        .with_target(true)
        .with_level(false)
        .with_line_number(true)
        .compact()
        .init();

    dotenvy::dotenv().ok();
    let setting = Settings::new()?;

    run(setting).await
}
