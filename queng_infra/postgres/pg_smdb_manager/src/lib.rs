mod db_svc;

use common_errors::prelude::PostgresDBError;
use diesel::{Connection, PgConnection};
use pg_smdb::run_smdb_db_migration;

pub struct PostgresSMDBManager {
    dbg: bool,
    conn: PgConnection,
}

impl PostgresSMDBManager {
    pub async fn new(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(false, url).await
    }

    pub async fn with_debug(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(true, url).await
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
    /// If successful, returns a PostgresSMDBManager instance.
    /// If the connection fails, returns a PostgresDBError indicating the failure.
    ///
    async fn build(dbg: bool, url: &str) -> Result<Self, PostgresDBError> {
        if dbg {
            println!("[PostgresSMDBManager]: Debug mode enabled");
            println!("[PostgresSMDBManager]: Connecting to Postgres database:",);
        }

        let mut conn = match PgConnection::establish(url) {
            Ok(conn) => conn,
            Err(e) => {
                return Err(PostgresDBError::ConnectionFailed(e.to_string()));
            }
        };

        match run_smdb_db_migration(&mut conn) {
            Ok(_) => {}
            Err(e) => {
                return Err(PostgresDBError::MigrationFailed(e.to_string()));
            }
        }

        Ok(Self { dbg, conn })
    }
}

impl PostgresSMDBManager {
    pub fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[PostgresSMDBManager]: {}", msg);
        }
    }
}
