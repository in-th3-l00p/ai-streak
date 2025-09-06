use argon2::password_hash::rand_core::OsRng;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use argon2::password_hash::SaltString;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use crate::http::get_router;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub hmac: Hmac<Sha256>,
    pub salt: SaltString,
    pub secret: Vec<u8>,
}

async fn initialize_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@localhost:5432/app_db".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to create the database connection");
    tracing::info!("database connection established");
    pool
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string());
    let secret = std::env::var("HMAC_SECRET")
        .expect("HMAC_SECRET is not set")
        .into_bytes();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    tracing::info!("auth service listening on port {}", port);
    axum::serve(
        listener,
        get_router(AppState {
            pool: initialize_db().await,
            hmac: Hmac::new_from_slice(&secret)
                .expect("HMAC_SECRET is not valid"),
            salt: SaltString::generate(&mut OsRng),
            secret, 
        }).into_make_service()
    ).await?;
    Ok(())
}
