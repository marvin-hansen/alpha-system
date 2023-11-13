use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Connect to the server
    let db: Surreal<Db> = Surreal::new::<Mem>(()).await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Build a new db manager
    // let dbm = db_manager::DBManager::new(db);
    //
    // let res = dbm.get_record().await?;
    //
    // assert!(res.is_none());

    Ok(())
}
