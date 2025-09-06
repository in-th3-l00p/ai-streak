use clap::Parser;

mod app;
mod http;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    if let Err(_) = dotenvy::dotenv() {
        tracing::warn!("failed to load .env file");
    }

    app::run().await?;
    Ok(())
}
