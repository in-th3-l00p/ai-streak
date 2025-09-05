use crate::db;
use std::sync::Arc;
use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<db::Database>,
}

pub fn app(db: Arc<db::Database>) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .with_state(AppState { db })
}

pub async fn run(db: Arc<db::Database>) -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    tracing::info!("auth service is running on port {}", port);
    axum::serve(listener, app(db).into_make_service())
        .await?;
    Ok(())
}
