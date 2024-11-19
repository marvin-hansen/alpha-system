mod integration_config;

use common_errors::PostgresDBError;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use pg_imdb::run_imdb_db_migration;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct PostgresIMDBManager {
    dbg: bool,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresIMDBManager {
    /// Asynchronously creates a new instance by building with the provided URL.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice representing the URL for the new instance.
    ///
    /// # Returns
    ///
    /// A Result containing the new instance or a `PostgresDBError`.
    ///
    pub async fn new(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(false, false, url).await
    }

    /// Asynchronously creates a new instance with debug mode enabled.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice representing the URL for the new instance.
    ///
    /// # Returns
    ///
    /// A Result containing the new instance or a `PostgresDBError`.
    ///
    pub async fn with_debug(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(true, false, url).await
    }

    /// Asynchronously initializes a connection with debug mode and test mode enabled.
    /// Test mode means, all database transactions will be rolled back and
    /// the DB connection closed automatically when the test instance of `PostgresIMDBManager` is dropped.
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
    /// or a `PostgresDBError`.
    ///
    pub async fn with_test_and_debug(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(true, true, url).await
    }

    /// Asynchronously creates a new instance from the provided pool with debug mode enabled.
    ///
    /// # Arguments
    ///
    /// * `pool` - A connection pool to use for the new instance.
    /// * `dbg` - A boolean indicating whether debug mode is enabled.
    ///
    /// # Returns
    ///
    /// A Result containing the new instance or a `PostgresDBError`.
    ///
    pub async fn with_pool_and_debug(
        pool: Pool<ConnectionManager<PgConnection>>,
        dbg: bool,
    ) -> Result<Self, PostgresDBError> {
        let conn = &mut pool
            .get()
            .expect("Failed to get a connection from the pool");

        run_imdb_db_migration(conn).expect("Failed to run IMDB migration");

        Ok(Self { dbg, pool })
    }
}

impl PostgresIMDBManager {
    async fn build(dbg: bool, test: bool, url: &str) -> Result<Self, PostgresDBError> {
        if dbg {
            println!("[PostgresIMDBManager]: Debug mode enabled");
            println!(
                "[PostgresIMDBManager]: Connecting to Postgres database: {}",
                &url
            );
        }

        let pool = postgres_common::build_pg_connection_pool_with_migration(
            test,
            dbg,
            url,
            run_imdb_db_migration,
        )
        .expect("[PostgresIMDBManager]: Failed to create Postgres connection pool");

        Ok(Self { dbg, pool })
    }

    pub(crate) fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool
            .get()
            .expect("[PostgresIMDBManager]: Failed to get connection from pool")
    }

    pub fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[PostgresIMDBManager]: {msg}");
        }
    }
}

impl Display for PostgresIMDBManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PostgresIMDBManager")
    }
}
