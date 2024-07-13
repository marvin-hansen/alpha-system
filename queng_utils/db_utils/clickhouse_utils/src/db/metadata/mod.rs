use klickhouse::Client;

mod db_setup;
mod db_teardown;
mod db_utils;
mod tables;

pub(crate) const ASSETS_TABLE: &str = "assets";
pub(crate) const EXCHANGES_TABLE: &str = "exchanges";
pub(crate) const INSTRUMENTS_TABLE: &str = "instruments";
pub(crate) const STATS_TABLE: &str = "stats";

pub(crate) const DB_TABLES: [&str; 4] = [
    ASSETS_TABLE,
    EXCHANGES_TABLE,
    INSTRUMENTS_TABLE,
    STATS_TABLE,
];

pub(crate) const DB_NAME: &str = "metadata";

#[derive(Clone)]
pub struct Metadata {
    dbg: bool,
    client: Client,
}

impl Metadata {
    pub fn new(client: Client, dbg: bool) -> Self {
        Self { client, dbg }
    }

    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ClickhouseUtil]:[DB]:[Metadata]: {}", s);
        }
    }
}
