mod db_svc;

use common_errors::prelude::PostgresDBError;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use pg_smdb::run_smdb_db_migration;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct PostgresSMDBManager {
    dbg: bool,
    pool: Pool<ConnectionManager<PgConnection>>,
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
    ///    If successful, returns a PostgresSMDBManager instance.
    ///    If the connection fails, returns a PostgresDBError indicating the failure.
    ///
    async fn build(dbg: bool, url: &str) -> Result<Self, PostgresDBError> {
        if dbg {
            println!("[PostgresSMDBManager]: Debug mode enabled");
            println!(
                "[PostgresSMDBManager]: Connecting to Postgres database: {}",
                &url
            );
        }

        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = match Pool::builder().test_on_check_out(true).build(manager) {
            Ok(pool) => pool,
            Err(e) => {
                return Err(PostgresDBError::ConnectionFailed(e.to_string()));
            }
        };

        if dbg {
            println!("[PostgresSMDBManager]: Run DB Migration",);
        }
        match run_smdb_db_migration(&mut pool.get().unwrap()) {
            Ok(_) => {}
            Err(e) => {
                return Err(PostgresDBError::MigrationFailed(e.to_string()));
            }
        }

        Ok(Self { dbg, pool })
    }
}

// impl PostgresSMDBManager {
//     pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
//         let mut conn = self.pool.get().expect("Failed to get connection from pool");
//         if self.test {
//             conn.begin_test_transaction()
//                 .expect("[PostgresSMDBManager]: Failed to begin test transaction");
//         };
//
//         conn
//     }
// }

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
