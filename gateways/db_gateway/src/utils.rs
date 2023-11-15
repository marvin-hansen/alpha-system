use surrealdb::{Error, Surreal};
use surrealdb::engine::local;

use components::prelude::DBManager;

pub async fn get_dbm() -> Result<DBManager, Error> {
    let db: Surreal<local::Db> = Surreal::new::<local::Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    let dbm = DBManager::new(db);

    Ok(dbm)
}
