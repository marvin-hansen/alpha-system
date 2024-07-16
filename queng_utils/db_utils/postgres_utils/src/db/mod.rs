use deadpool_diesel::postgres::Pool;

mod specs;

// pub(crate) const SERVICE_TABLE: &str = "service";
// pub(crate) const PORTFOLIO_TABLE: &str = "portfolio";

pub struct Specs {
    dbg: bool,
    pool: Pool,
}

impl Specs {
    pub fn new(dbg: bool, pool: Pool) -> Self {
        Self { dbg, pool }
    }

    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ClickhouseUtil]:[DB]:[Specs]: {}", s);
        }
    }
}
