use surrealdb::engine::local;
use surrealdb::Surreal;

use common::prelude::DBConfig;

mod db_svc;

#[derive(Clone)]
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
