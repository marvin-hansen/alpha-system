mod db;
mod errors;
pub mod prelude;
mod query_utils;
mod types;

use crate::db::Specs;
use crate::prelude::PostgresUtilError;
use deadpool_diesel::postgres::{Manager, Pool};

pub struct PostgresUtil {
    dbg: bool,
    pub specs: Specs,
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
        }

        let pool = Self::get_pg_pool(dsn, 5)
            .await
            .expect("[PostgresUtil]: Failed to construct database connection pool");

        let specs = Specs::new(dbg, pool);

        Ok(Self { dbg, specs })
    }

    async fn get_pg_pool(database_url: &str, max_size: usize) -> Result<Pool, PostgresUtilError> {
        //
        let manager = Manager::new(database_url.to_string(), deadpool_diesel::Runtime::Tokio1);

        match Pool::builder(manager).max_size(max_size).build() {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresUtilError::from(e.to_string())),
        }
    }
}

impl PostgresUtil {
    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ClickhouseUtil]: {}", s);
        }
    }
}
