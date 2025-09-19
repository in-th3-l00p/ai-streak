use clap::Parser;
use crate::cli::ExecutionMode;

mod app;
mod http;
mod service;
mod cli;
mod streaming;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    if let Err(_) = dotenvy::dotenv() {
        tracing::warn!("failed to load .env file");
    }

    let args = cli::Args::parse();
    if let ExecutionMode::Server = args.mode {
        app::run().await?;
    } else if let ExecutionMode::Seeding = args.mode {
        cli::seeding::seed().await;
    }

    Ok(())
}
