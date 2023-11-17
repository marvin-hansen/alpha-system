use surrealdb::engine::local;
use surrealdb::Surreal;

mod db_svc;

#[derive(Clone)]
pub struct DBManager {
    db: Surreal<local::Db>,
}

impl DBManager {
    pub fn new(db: Surreal<local::Db>) -> Self {
        // local DB is either in memory or flat file on disk so it's always connected
        Self { db }
    }
}
