use surrealdb::engine::local::Db;
use surrealdb::Error;
use surrealdb::Surreal;

pub struct DBManager {
    db: Surreal<Db>,
}

impl DBManager {
    pub fn new(db: Surreal<Db>) -> Self {
        Self { db }
    }
}

impl DBManager {
    pub async fn get_record(&self) -> Result<Option<String>, Error> {
        Ok(None)
    }
}