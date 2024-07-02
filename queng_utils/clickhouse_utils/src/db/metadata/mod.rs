use klickhouse::Client;

// mod symbols;
mod db_setup;
mod db_teardown;
mod db_utils;
mod tables;

pub(crate) const DB_NAME: &'static str = "metadata";
pub(crate) const DB_TABLES: [&'static str; 4] = ["assets", "exchanges", "instruments", "stats"];

#[derive(Clone)]
pub struct Metadata {
    client: Client,
}

impl Metadata {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
