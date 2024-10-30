mod asset;
mod exchange;
mod instrument;
mod stat;

use common_errors::prelude::PostgresDBError;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use pg_mddb::run_mddb_migration;
use postgres_common::TestConnectionCustomizer;

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
    pub async fn test_with_debug(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(true, true, url).await
    }

    // Builds a new instance of PostgresMDDBManager
    //
    // # Arguments
    //
    // * `dbg` - A boolean flag indicating whether debug mode is enabled.
    // * `test` - A boolean flag indicating whether test mode is enabled.
    // * `url` - A string slice representing the URL for the new instance.
    //
    // # Returns
    //
    // A Result containing the new instance or a PostgresDBError.
    //
    async fn build(dbg: bool, test: bool, url: &str) -> Result<Self, PostgresDBError> {
        if dbg {
            println!("[PostgresMDDBManager]: Debug mode enabled");
            println!(
                "[PostgresMDDBManager]: Connecting to Postgres database: {}",
                &url
            );
        }

        let pool = if test {
            Pool::builder()
                .test_on_check_out(true)
                .max_size(1)
                .connection_customizer(Box::new(TestConnectionCustomizer))
                .build(ConnectionManager::<PgConnection>::new(url))
                .expect("Failed to create pool with test transaction")
        } else {
            Pool::builder()
                .test_on_check_out(true)
                .max_size(10)
                .build(ConnectionManager::<PgConnection>::new(url))
                .expect("Failed to create pool")
        };

        // For tests, we most likely have a blank DB,
        // thus run migration to create the schema first.
        if test {
            if dbg {
                println!("[PostgresMDDBManager]: Run DB Migration",);
            }
            match run_mddb_migration(&mut pool.get().unwrap()) {
                Ok(_) => {}
                Err(e) => {
                    return Err(PostgresDBError::MigrationFailed(e.to_string()));
                }
            }
        }

        Ok(Self { dbg, pool })
    }
}

impl PostgresMDDBManager {
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
