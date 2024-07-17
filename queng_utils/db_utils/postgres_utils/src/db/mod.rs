mod specs;

use tokio_postgres::Client;

pub(crate) const DB_NAME: &str = "specs";

pub(crate) const SERVICE_TABLE: &str = "service";
pub(crate) const PORTFOLIO_TABLE: &str = "portfolio";

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
