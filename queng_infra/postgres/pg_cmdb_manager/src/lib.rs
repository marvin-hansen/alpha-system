// Unsafe code must be explicitly enabled to use it.
#[deny(unsafe_code)]
//
use common_errors::prelude::PostgresDBError;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{Connection, PgConnection};
use pg_cmdb::run_cmdb_db_migration;
use std::fmt::Display;

mod db_prtf;

#[derive(Clone, Debug)]
pub struct PostgresCMDBManager {
    dbg: bool,
    test: bool,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresCMDBManager {
    pub async fn new(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(false, false, true, url).await
    }

    pub async fn with_debug(url: &str, migration: bool) -> Result<Self, PostgresDBError> {
        Self::build(true, false, migration, url).await
    }

    pub async fn with_test_and_debug(url: &str, migration: bool) -> Result<Self, PostgresDBError> {
        Self::build(true, true, migration, url).await
    }

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
    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
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
