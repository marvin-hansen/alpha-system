// Unsafe code must be explicitly enabled to use it.
#[deny(unsafe_code)]
//
use common_errors::prelude::PostgresDBError;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use pg_cmdb::run_cmdb_db_migration;
use std::fmt::Display;

mod db_prtf;
#[derive(Clone, Debug)]
pub struct PostgresCMDBManager {
    dbg: bool,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresCMDBManager {
    pub async fn new(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(false, url).await
    }

    pub async fn with_debug(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(true, url).await
    }
    async fn build(dbg: bool, url: &str) -> Result<Self, PostgresDBError> {
        if dbg {
            println!("[PostgresCMDBManager]: Debug mode enabled");
            println!(
                "[PostgresCMDBManager]: Connecting to Postgres database: {}",
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
            println!("[PostgresCMDBManager]: Run DB Migration",);
        }
        match run_cmdb_db_migration(&mut pool.get().unwrap()) {
            Ok(_) => {}
            Err(e) => {
                return Err(PostgresDBError::MigrationFailed(e.to_string()));
            }
        }

        Ok(Self { dbg, pool })
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
