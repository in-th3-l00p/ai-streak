use sqlx::postgres::{PgPoolOptions, PgPool};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL")
                .unwrap_or("postgres://localhost:5432/my_database".to_string()
            ))
            .await?;
        Ok(Self { pool })
    }
}