mod asset;
mod exchange;
mod instrument;
mod instrument_queries;
mod stat;

use common_errors::prelude::PostgresDBError;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use pg_mddb::run_mddb_migration;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct PostgresMDDBManager {
    dbg: bool,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresMDDBManager {
    /// Asynchronously creates a new instance by building with the provided URL.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice representing the URL for the new instance.
    ///
    /// # Returns
    ///
    /// A Result containing the new instance or a PostgresDBError.
    ///
    pub async fn new(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(false, false, url).await
    }

    /// Asynchronously initializes a connection with debug mode enabled.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice representing the URL for the connection.
    ///
    /// # Returns
    ///
    /// A Result containing the initialized connection or a PostgresDBError.
    ///
    pub async fn with_debug(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(true, false, url).await
    }

    /// Asynchronously initializes a connection with debug mode and test mode enabled.
    /// Test mode means, all database transactions will be rolled back and
    /// the DB connection closed automatically when the test instance of PostgresMDDBManager is dropped.
    ///
    /// A full DB schema migration happens during initialization.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice representing the URL for the new instance.
    ///
    /// # Returns
    ///
    /// A Result containing the new instance with a test transaction
    /// or a PostgresDBError.
    ///
    pub async fn with_test_and_debug(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(true, true, url).await
    }

    async fn build(dbg: bool, test: bool, url: &str) -> Result<Self, PostgresDBError> {
        if dbg {
            println!("[PostgresMDDBManager]: Debug mode enabled");
            println!(
                "[PostgresMDDBManager]: Connecting to Postgres database: {}",
                &url
            );
        }

        let pool = postgres_common::build_pg_connection_pool(test, dbg, url, run_mddb_migration)
            .expect("[PostgresMDDBManager]: Failed to create Postgres connection pool");

        Ok(Self { dbg, pool })
    }

    /// Returns a connection from the connection pool.
    pub(crate) fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to get connection from pool")
    }
}

impl PostgresMDDBManager {
    pub fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[PostgresMDDBManager]: {}", msg);
        }
    }
}

impl Display for PostgresMDDBManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PostgresMDDBManager")
    }
}
