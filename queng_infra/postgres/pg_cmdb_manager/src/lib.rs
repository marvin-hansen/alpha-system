use common_errors::PostgresDBError;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use pg_cmdb::run_cmdb_db_migration;
use std::fmt::Display;

mod portfolio_config;

#[derive(Clone, Debug)]
pub struct PostgresCMDBManager {
    dbg: bool,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresCMDBManager {
    ///
    /// Asynchronously creates a new instance by building with the provided URL.
    /// Tests for database migration and performs it automatically if required.
    /// Use this constructor by default.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice representing the URL for the database connection.
    ///
    /// # Returns
    ///
    /// A Result containing the newly created instance or a PostgresDBError.
    ///
    pub async fn new(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(false, false, url).await
    }

    ///
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

    ///
    /// Asynchronously initializes a connection to the Postgres database
    /// with test and debug options enabled.
    ///
    /// Note, in test mode, each DB method is performed in a test transaction that aborts
    /// at the end of the method call so that no changes are committed to the DB.
    /// Use this for testing only.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice representing the URL of the Postgres database.
    ///
    /// # Returns
    ///
    /// A Result containing the initialized connection or a PostgresDBError if an error occurs.
    ///
    pub async fn with_test_and_debug(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(true, true, url).await
    }

    /// Asynchronously initializes a PostgresCMDBManager with a given connection pool and debug mode.
    ///
    /// # Arguments
    ///
    /// * `pool` - A connection pool to the Postgres database.
    /// * `dbg` - A boolean indicating whether debug mode is enabled.
    ///
    /// # Returns
    ///
    /// A Result containing the initialized PostgresCMDBManager instance or a PostgresDBError.
    ///
    pub async fn with_pool_and_debug(
        pool: Pool<ConnectionManager<PgConnection>>,
        dbg: bool,
    ) -> Result<Self, PostgresDBError> {
        let conn = &mut pool
            .get()
            .expect("Failed to get a connection from the pool");

        run_cmdb_db_migration(conn).expect("Failed to run CMDB database migration");

        Ok(Self { dbg, pool })
    }
}

impl PostgresCMDBManager {
    async fn build(dbg: bool, test: bool, url: &str) -> Result<Self, PostgresDBError> {
        if dbg {
            println!("[PostgresCMDBManager]: Debug mode enabled");
            println!(
                "[PostgresCMDBManager]: Connecting to Postgres database: {}",
                &url
            );
        }

        let pool = postgres_common::build_pg_connection_pool_with_migration(
            test,
            dbg,
            url,
            run_cmdb_db_migration,
        )
        .expect("[PostgresCMDBManager]: Failed to create Postgres connection pool");

        Ok(Self { dbg, pool })
    }

    /// Returns a connection from the connection pool.
    pub(crate) fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to get connection from pool")
    }
}
impl PostgresCMDBManager {
    pub fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[PostgresCMDBManager]: {}", msg);
        }
    }
}

impl Display for PostgresCMDBManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PostgresCMDBManager")
    }
}
