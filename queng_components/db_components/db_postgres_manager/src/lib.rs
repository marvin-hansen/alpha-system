use std::fmt::{Debug, Display, Formatter};
use tokio::task::JoinHandle;
use tokio_postgres::{Client, NoTls};

use common_database::prelude::PostgresDBConfig;

use crate::error::PostgresDBError;

mod db_prtf;
mod db_svc;
mod db_util;
pub mod error;
pub mod prelude;

#[derive(Debug)]
pub struct PostgresDBManager {
    dbg: bool,
    // pool: Pool<ConnectionManager<PgConnection>>,
    client: Client,
    handle: JoinHandle<()>,
}

impl PostgresDBManager {
    /// Asynchronously builds a new `PostgresDBManager` instance.
    ///
    /// # Arguments
    ///
    /// * `pg_config` - The `PostgresDBConfig` for the connection.
    ///
    /// # Returns
    ///
    /// A result containing a `PostgresDBManager` instance on success, or a
    /// `PostgresDBError` on failure.
    ///
    pub async fn new(pg_config: &PostgresDBConfig) -> Result<Self, PostgresDBError> {
        let tsn = &pg_config.tsn();
        Self::build(false, tsn).await
    }

    /// Asynchronously builds a new `PostgresDBManager` instance with debug mode enabled.
    ///
    /// # Arguments
    ///
    /// * `pg_config` - The `PostgresDBConfig` for the connection.
    ///
    /// # Returns
    ///
    /// A result containing a `PostgresDBManager` instance on success, or a
    /// `PostgresDBError` on failure.
    ///
    pub async fn with_debug(pg_config: &PostgresDBConfig) -> Result<Self, PostgresDBError> {
        let tsn = &pg_config.tsn();
        Self::build(true, tsn).await
    }

    /// Asynchronously builds a new `PostgresDBManager` instance.
    ///
    /// # Arguments
    ///
    /// * `dbg` - Whether to enable debug mode.
    /// * `tsn` - The target Postgres server name.
    ///
    /// # Returns
    ///
    /// A result containing a `PostgresDBManager` instance on success, or a
    /// `PostgresDBError` on failure.
    ///
    async fn build(dbg: bool, tsn: &str) -> Result<Self, PostgresDBError> {
        if dbg {
            println!("[PostgresDBManager]: Debug mode enabled");
            println!("[PostgresDBManager]: Connecting to Postgres database:",);
        }

        let (client, connection) = match tokio_postgres::connect(tsn, NoTls).await {
            Ok((client, connection)) => (client, connection),
            Err(e) => {
                return Err(PostgresDBError::ConnectionFailed(e.to_string()));
            }
        };

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        let handle = tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!(
                    "[PostgresDBManager]: Tokio/Postgres failed to spwan connection task: {}",
                    e
                );
            }
        });

        Ok(Self {
            dbg,
            client,
            handle,
        })
    }
}

impl PostgresDBManager {
    pub async fn close(&self) {
        self.dbg_print("Closing Postgres connection via Tokio task handle");
        // https://stackoverflow.com/questions/67160923/how-can-you-close-a-tokio-postgres-connection
        self.handle.abort();
    }
}

impl PostgresDBManager {
    fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[PostgresDBManager]: {}", msg);
        }
    }
}

impl Display for PostgresDBManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PostgresDBManager:",)
    }
}
