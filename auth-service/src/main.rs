use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    if let Result::Err(_) = dotenvy::dotenv() {
        tracing::warn!("error loading .env file");
    }
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    tracing::info!("server is running on port {}", port);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tracing::debug!("new connection: {}", socket.peer_addr().unwrap());
    }
}
