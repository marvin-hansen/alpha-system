use std::fmt::Debug;
use surrealdb::engine::local;
use surrealdb::Surreal;

use common::prelude::DBConfig;

mod db_cfg;
mod db_svc;

#[derive(Clone, Debug)]
pub struct DBManager {
    db: Surreal<local::Db>,
}

impl DBManager {
    pub async fn new_offline(db_config: &DBConfig) -> Self {
        let ns = db_config.db_namespace();
        let db_name = db_config.db_name();

        let db: Surreal<local::Db> = Surreal::new::<local::Mem>(()).await.unwrap();
        db.use_ns(ns).use_db(db_name).await.unwrap();

        Self { db }
    }
}

impl Default for DBManager {
    fn default() -> Self {
        // How do I synchronously return a value calculated in an asynchronous Future?
        // https://stackoverflow.com/questions/52521201/how-do-i-synchronously-return-a-value-calculated-in-an-asynchronous-future
        use futures::executor; // 0.3.1
        let db_config = DBConfig::default();
        let db = executor::block_on(Self::new_offline(&db_config));

        db
    }
}
