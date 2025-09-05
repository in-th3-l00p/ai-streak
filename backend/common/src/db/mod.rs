use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or(format!("postgres://postgres:postgres@localhost:5432/app_db"));
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        tracing::info!("database connection established");
        Ok(Self { pool })
    }
}
