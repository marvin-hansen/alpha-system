use error::spec_db_error::SpecDBError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

mod db_portfolio;
mod db_service;
mod error;
pub mod prelude;
mod types;
mod utils;

pub struct SpecDBManager {
    pool: Pool<Postgres>,
}

impl SpecDBManager {
    pub async fn new() -> Self {
        let url = "postgres://postgres:password@localhost/test";
        Self::build(url).await.unwrap()
    }

    pub async fn with_debug() -> Self {
        let url = "postgres://postgres:password@localhost/test";
        Self::build(url).await.unwrap()
    }

    pub async fn build(url: &str) -> Result<Self, SpecDBError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .expect("Failed to create a connection pool");

        Ok(Self { pool })
    }
}

impl SpecDBManager {
    /// Gracefully closes all remaining DB connection.
    /// Call during shutdown of your application to ensure
    /// the Databases closes all connections correctly.
    pub async fn close(&self) {
        self.pool.close().await
    }
}
