mod db_count;
mod db_error;
mod db_import;
mod db_setup;
mod db_teardown;
mod db_verify;
pub mod prelude;

use crate::prelude::PostgresUtilError;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

#[derive(Debug, Clone)]
pub struct PostgresUtil {
    dbg: bool,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresUtil {
    pub async fn new(dsn: &str) -> Result<Self, PostgresUtilError> {
        Self::build(false, dsn).await
    }

    pub async fn with_debug(dsn: &str) -> Result<Self, PostgresUtilError> {
        Self::build(true, dsn).await
    }

    async fn build(dbg: bool, dsn: &str) -> Result<Self, PostgresUtilError> {
        if dbg {
            println!("[PostgresUtil]: Debug mode enabled");
            println!("[PostgresUtil]: Connecting to Postgres database: {}", dsn);
        }

        let pool = postgres_connection_pool(dsn)
            .await
            .expect("[PostgresUtil]: Failed to connect to Postgres database");

        Ok(Self { dbg, pool })
    }
}

async fn postgres_connection_pool(
    dsn: &str,
) -> Result<Pool<ConnectionManager<PgConnection>>, PostgresUtilError> {
    let manager = ConnectionManager::<PgConnection>::new(dsn);
    match Pool::builder().test_on_check_out(true).build(manager) {
        Ok(pool) => Ok(pool),
        Err(e) => Err(PostgresUtilError::new(format!(
            "[PostgresUtil]: Failed to build connection pool: {}",
            e
        ))),
    }
}

impl PostgresUtil {
    fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[PostgresUtil]: {}", msg);
        }
    }
}
