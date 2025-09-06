use crate::http::get_router;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
}

impl AppState {
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

    pub async fn new() -> Self {
        let pool = Arc::new(Self::initialize_db().await);
        Self { }
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8081".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    tracing::info!("notes service listening on port {}", port);
    axum::serve(
        listener,
        get_router(AppState::new().await).into_make_service(),
    )
    .await?;
    Ok(())
}
