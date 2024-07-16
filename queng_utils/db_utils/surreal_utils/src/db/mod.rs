mod specs;

// pub(crate) const SERVICE_TABLE: &str = "service";
// pub(crate) const PORTFOLIO_TABLE: &str = "portfolio";

#[derive(Clone)]
pub struct Specs {
    dbg: bool,
}

impl Specs {
    pub fn new(dbg: bool) -> Self {
        Self { dbg }
    }

    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ClickhouseUtil]:[DB]:[Specs]: {}", s);
        }
    }
}
