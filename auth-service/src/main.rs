mod http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    if let Result::Err(_) = dotenvy::dotenv() {
        tracing::warn!("error loading .env file");
    }

    http::run().await
}
