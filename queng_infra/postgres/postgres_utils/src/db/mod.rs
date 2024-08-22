use tokio_postgres::Client;

mod all_db_setup;
mod all_db_teardown;
mod all_db_verify;
mod specs;

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
