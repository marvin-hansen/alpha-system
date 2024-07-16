use diesel_async::AsyncPgConnection;

mod specs;

// pub(crate) const SERVICE_TABLE: &str = "service";
// pub(crate) const PORTFOLIO_TABLE: &str = "portfolio";

pub struct Specs {
    dbg: bool,
    db: AsyncPgConnection,
}

impl Specs {
    pub fn new(dbg: bool, db: AsyncPgConnection) -> Self {
        Self { dbg, db }
    }

    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ClickhouseUtil]:[DB]:[Specs]: {}", s);
        }
    }
}
