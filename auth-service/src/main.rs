mod http;
mod db;

use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    if let Result::Err(_) = dotenvy::dotenv() {
        tracing::warn!("error loading .env file");
    }

    match db::Database::new().await {
        Ok(db) => {
            tracing::info!("database connection started");
            let db = Arc::new(db);
            http::run(db).await?;
            Ok(())
        }
        Err(e) => {
            tracing::error!("error connecting to database: {}", e);
            Ok(())
        }
    }
}
