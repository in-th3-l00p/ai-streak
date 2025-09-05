use tokio::net::TcpListener;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
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