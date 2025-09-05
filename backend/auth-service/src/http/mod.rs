pub mod routes;

use crate::db;
use axum::{Router, routing::get};
use std::sync::Arc;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<db::Database>,
    pub hmac: Hmac<Sha256>,
    pub salt: SaltString,
}

pub fn app(db: Arc<db::Database>, secret: String) -> Router {
    Router::<AppState>::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/api", routes::router())
        .with_state(AppState {
            db,
            hmac: Hmac::new_from_slice(secret.as_bytes())
                .expect("HMAC_SECRET is not valid"),
            salt: SaltString::generate(&mut OsRng),
        })
}

pub async fn run(db: Arc<db::Database>) -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let secret = std::env::var("HMAC_SECRET").expect("HMAC_SECRET is not set");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    tracing::info!("auth service is running on port {}", port);
    axum::serve(listener, app(db, secret).into_make_service()).await?;
    Ok(())
}
