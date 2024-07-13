mod db;
mod errors;
mod prelude;
mod types;

use crate::prelude::SurrealUtilError;
use common::prelude::SurrealDBConfig;
use db_surreal_manager::SurrealDBManager;

pub struct SurrealUtil {
    dbg: bool,
    db: SurrealDBManager,
}

impl SurrealUtil {
    pub async fn new(db_config: &SurrealDBConfig) -> Result<Self, SurrealUtilError> {
        Self::build(false, db_config).await
    }

    pub async fn with_debug(db_config: &SurrealDBConfig) -> Result<Self, SurrealUtilError> {
        Self::build(true, db_config).await
    }

    async fn build(dbg: bool, db_config: &SurrealDBConfig) -> Result<Self, SurrealUtilError> {
        let db = SurrealDBManager::new(db_config)
            .await
            .expect("Failed to build SurrealDBManager");

        Ok(Self { dbg, db })
    }
}

impl SurrealUtil {
    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[SurrealUtil]: {}", s);
        }
    }
}
