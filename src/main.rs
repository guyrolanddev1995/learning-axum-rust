use axumApp::config::settings::Settings;
use axumApp::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    dotenvy::dotenv().ok();
    let setting = Settings::new()?;

    run(setting).await
}