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

    let db = Arc::new(db::Database::new().await?);
    http::run(db).await
}
