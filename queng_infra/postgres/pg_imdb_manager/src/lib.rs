mod integration_config;

use common_errors::prelude::PostgresDBError;
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
    pub async fn new(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(false, false, url).await
    }

    pub async fn with_debug(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(true, false, url).await
    }

    pub async fn with_test_and_debug(url: &str) -> Result<Self, PostgresDBError> {
        Self::build(true, true, url).await
    }

    pub async fn with_pool_and_debug(
        dbg: bool,
        pool: Pool<ConnectionManager<PgConnection>>,
    ) -> Result<Self, PostgresDBError> {
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

        let pool = postgres_common::build_pg_connection_pool(test, dbg, url, run_imdb_db_migration)
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
            println!("[PostgresIMDBManager]: {}", msg);
        }
    }
}

impl Display for PostgresIMDBManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PostgresIMDBManager")
    }
}
