use common_errors::prelude::PostgresDBError;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{Connection, PgConnection};
use pg_cmdb::run_cmdb_db_migration;
use std::fmt::Display;

mod portfolio_config;

#[derive(Clone, Debug)]
pub struct PostgresCMDBManager {
    dbg: bool,
    test: bool,
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
        Self::build(false, false, true, url).await
    }

    ///
    /// Asynchronously creates a new instance by building with the provided URL.
    /// Does not test or perform database migration. Use this constructor
    /// when you are 100% certain that the DB already has been migrated.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice representing the URL for the database connection.
    ///
    /// # Returns
    ///
    /// A Result containing the newly created instance or a PostgresDBError.
    ///
    pub async fn new_no_migration(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(false, false, false, url).await
    }

    ///
    /// Asynchronously initializes a connection with debug mode enabled.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice representing the URL for the connection.
    /// * `migration` - A boolean indicating whether to run migrations.
    ///
    /// # Returns
    ///
    /// A Result containing the initialized connection or a PostgresDBError.
    ///
    pub async fn with_debug(url: &str, migration: bool) -> Result<Self, PostgresDBError> {
        Self::build(true, false, migration, url).await
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
    /// * `migration` - A boolean indicating whether to run database migration.
    ///
    /// # Returns
    ///
    /// A Result containing the initialized connection or a PostgresDBError if an error occurs.
    ///
    pub async fn with_test_and_debug(url: &str, migration: bool) -> Result<Self, PostgresDBError> {
        Self::build(true, true, migration, url).await
    }

    ///
    /// Asynchronously builds a PostgresCMDBManager instance based on the provided configuration parameters.
    ///
    /// This function handles the following workflow:
    /// 1. Establishes a connection pool to the Postgres database using the provided URL.
    /// 2. Optionally enables debug mode to print debug messages.
    /// 3. Performs database migration if the migration parameter is set to true.
    ///
    /// # Arguments
    ///
    /// * `dbg` - A boolean indicating whether debug mode is enabled.
    /// * `test` - A boolean indicating whether testing mode is enabled.
    /// * `migration` - A boolean indicating whether to run database migration.
    /// * `url` - A reference to a string representing the URL of the Postgres database.
    ///
    /// # Returns
    ///
    /// A Result containing the constructed PostgresCMDBManager instance
    /// or a PostgresDBError if an error occurs during the process.
    ///
    async fn build(
        dbg: bool,
        test: bool,
        migration: bool,
        url: &str,
    ) -> Result<Self, PostgresDBError> {
        if dbg {
            println!("[PostgresCMDBManager]: Debug mode enabled");
            println!(
                "[PostgresCMDBManager]: Connecting to Postgres database: {}",
                &url
            );
        }

        let pool = match Pool::builder()
            .test_on_check_out(true)
            .build(ConnectionManager::<PgConnection>::new(url))
        {
            Ok(pool) => pool,
            Err(e) => {
                return Err(PostgresDBError::ConnectionFailed(e.to_string()));
            }
        };

        if migration {
            if dbg {
                println!("[PostgresCMDBManager]: Run DB Migration",);
            }
            match run_cmdb_db_migration(&mut pool.get().unwrap()) {
                Ok(_) => {}
                Err(e) => {
                    return Err(PostgresDBError::MigrationFailed(e.to_string()));
                }
            }
        }

        Ok(Self { dbg, test, pool })
    }
}

impl PostgresCMDBManager {
    ///
    /// Retrieves a database connection from the pool.
    ///
    /// If in test mode, begins a test transaction and runs CMDB database migration;
    /// Note, a test transaction abort at the end of the function call so that no changes
    /// are committed to the DB. Use for testing only.
    ///
    /// # Returns
    ///
    /// A pooled connection from the pool.
    pub(crate) fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        let mut conn = self.pool.get().expect("Failed to get connection from pool");

        if self.test {
            conn.begin_test_transaction()
                .expect("[PostgresCMDBManager]: Failed to begin test transaction");

            // We cannot assume the DB was migrated prior to a test, so we check here.
            run_cmdb_db_migration(&mut conn)
                .expect("[PostgresCMDBManager]: Failed to run migration for test");
        };

        conn
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
