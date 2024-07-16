mod db;
mod errors;
pub mod prelude;
mod query_utils;
mod types;

use crate::db::Specs;
use crate::prelude::PostgresUtilError;
use diesel_async::{AsyncConnection, AsyncPgConnection};

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

        let db = Self::get_pg_client(dsn)
            .await
            .expect("[PostgresUtil]: Failed to construct database client");

        let specs = Specs::new(dbg, db);

        Ok(Self { dbg, specs })
    }

    async fn get_pg_client(database_url: &str) -> Result<AsyncPgConnection, PostgresUtilError> {
        match AsyncPgConnection::establish(database_url).await {
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
