use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub mongodb_url: String
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|port| port.parse().ok())
                .unwrap_or(8080),
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),

            mongodb_url: std::env::var("MONGO_URL").expect("MONGO_URL must be set")
        })
    }
}