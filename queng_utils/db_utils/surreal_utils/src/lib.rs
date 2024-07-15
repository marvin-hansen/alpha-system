mod db;
mod errors;
pub mod prelude;
mod query_utils;
mod types;

use crate::db::Specs;
use crate::prelude::SurrealUtilError;
use common_config::prelude::SurrealDBConfig;
use db_surreal_manager::SurrealDBManager;

pub struct SurrealUtil {
    pub specs: Specs,
}

impl SurrealUtil {
    pub async fn new(db_config: &SurrealDBConfig) -> Result<Self, SurrealUtilError> {
        Self::build(false, db_config).await
    }

    pub async fn with_debug(db_config: &SurrealDBConfig) -> Result<Self, SurrealUtilError> {
        Self::build(true, db_config).await
    }

    async fn build(dbg: bool, db_config: &SurrealDBConfig) -> Result<Self, SurrealUtilError> {
        if dbg {
            println!("[ClickhouseUtil]: Debug mode enabled");
        }

        let db = SurrealDBManager::new(db_config)
            .await
            .expect("Failed to build SurrealDBManager");

        let specs = Specs::new(dbg, db);

        Ok(Self { specs })
    }
}
