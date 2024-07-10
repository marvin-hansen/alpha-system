use klickhouse::Client;

mod db_setup;

mod db_teardown;
mod db_utils;
mod tables;
pub(crate) const DB_NAME: &'static str = "specs";
//
// pub(crate) const DB_TABLES: [&'static str; 1] = ["services"];

#[derive(Clone)]
pub struct Specs {
    dbg: bool,
    client: Client,
}

impl Specs {
    pub fn new(client: Client, dbg: bool) -> Self {
        Self { client, dbg }
    }

    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ClickhouseUtil]:[DB]:[Specs]: {}", s);
        }
    }
}
