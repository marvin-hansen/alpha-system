mod service;

use common_errors::prelude::PostgresDBError;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use pg_smdb::run_smdb_db_migration;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct PostgresSMDBManager {
    dbg: bool,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresSMDBManager {
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

    /// Creates a new PostgresSMDBManager instance.
    ///
    /// # Arguments
    ///
    /// * `dbg` - If true, enables debug mode which prints debug messages.
    /// * `url` - The database connection URL.
    ///
    /// # Returns
    ///
    /// * `Result<Self, PostgresDBError>` - A result indicating success or failure.
    ///    If successful, returns a PostgresSMDBManager instance.
    ///    If the connection fails, returns a PostgresDBError indicating the failure.
    ///
    async fn build(dbg: bool, test: bool, url: &str) -> Result<Self, PostgresDBError> {
        if dbg {
            println!("[PostgresSMDBManager]: Debug mode enabled");
            println!(
                "[PostgresSMDBManager]: Connecting to Postgres database: {}",
                &url
            );
        }

        let pool = postgres_common::build_pg_connection_pool(test, dbg, url, run_smdb_db_migration)
            .expect("[PostgresSMDBManager]: Failed to create Postgres connection pool");

        Ok(Self { dbg, pool })
    }

    /// Returns a connection from the connection pool.
    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to get connection from pool")
    }
}

impl PostgresSMDBManager {
    pub fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[PostgresSMDBManager]: {}", msg);
        }
    }
}

impl Display for PostgresSMDBManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PostgresSMDBManager")
    }
}
