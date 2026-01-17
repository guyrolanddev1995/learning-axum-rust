use mongodb::{Client, Database};

#[derive(Clone)]
pub struct MongoDbConnection {
    db: Database
}

impl MongoDbConnection {
    pub async fn new(uri: &str) -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str(uri).await?;

        client.database("admin")
            .run_command(mongodb::bson::doc! {"ping": 1})
            .await?;

        tracing::info!("Connected to MongoDB");

        let db = client.database("pos");
        Ok(Self { db })
    }

    pub fn database(&self) -> &Database {
        &self.db
    }
}
