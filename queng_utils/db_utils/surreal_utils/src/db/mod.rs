use db_surreal_manager::SurrealDBManager;

mod specs;
pub(crate) const DB_NAME: &str = "specs";

pub(crate) const SERVICES_TABLE: &str = "services";

#[derive(Clone)]
pub struct Specs {
    dbg: bool,
    db: SurrealDBManager,
}

impl Specs {
    pub fn new(dbg: bool, db: SurrealDBManager) -> Self {
        Self { dbg, db }
    }

    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ClickhouseUtil]:[DB]:[Specs]: {}", s);
        }
    }
}
