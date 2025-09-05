use crate::db;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn run(db: Arc<db::Database>) -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    tracing::info!("notes service is running on port {}", port);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tracing::debug!("new connection: {}", socket.peer_addr().unwrap());
    }
}
