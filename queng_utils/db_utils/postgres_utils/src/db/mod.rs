mod all_db_constants;
mod all_db_setup;
mod all_db_teardown;
mod common_ddl;
mod common_queries;
mod specs;

use tokio_postgres::Client;

pub struct Specs {
    dbg: bool,
    db: Client,
}

impl Specs {
    pub fn new(dbg: bool, db: Client) -> Self {
        Self { dbg, db }
    }

    pub(crate) fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[PostgresUtil]:[Specs]: {}", s);
        }
    }
}
