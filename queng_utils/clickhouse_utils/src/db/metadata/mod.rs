use klickhouse::Client;

// mod symbols;
mod import;
mod info;
mod setup;
mod tables;
mod teardown;
mod utils;

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
