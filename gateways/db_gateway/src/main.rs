use surrealdb::engine::local;
use surrealdb::Error;
use surrealdb::Surreal;

use components::prelude::{DBManager, Status};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {

    // Build a new db manager
    let dbm = get_dbm().await.unwrap();

    let status = dbm.status().await.unwrap();
    assert_eq!(status, Status::Connected);


    Ok(())
}

async fn get_dbm() -> Result<DBManager, Error> {
    let db: Surreal<local::Db> = Surreal::new::<local::Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    let dbm = DBManager::new(db);

    Ok(dbm)
}
