pub mod domain;

use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            pool: PgPoolOptions::new()
                .max_connections(5)
                .connect(
                    &std::env::var("DATABASE_URL").unwrap_or(
                        "postgres://postgres:postgres@localhost:5432/app_db".to_string(),
                    ),
                )
                .await?,
        })
    }
}
