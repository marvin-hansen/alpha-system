mod db;

pub mod error;
pub(crate) mod fields;

pub(crate) mod query_utils;
mod setup;
mod teardown;
pub mod types;

use crate::db::metadata::Metadata;
use crate::db::specs::Specs;
use klickhouse::{Client, ClientOptions};

/// Creates a new clickhouse to connect to the provided DSN.
/// DSN format: url:port
pub async fn get_clickhouse_client(dsn: String) -> Client {
    let client = Client::connect(dsn.clone(), ClientOptions::default())
        .await
        .expect(format!("[get_clickhouse_client]: Failed to connect to {}", &dsn).as_str());

    client
}

pub struct ClickhouseUtil {
    dbg: bool,
    client: Client,
    metadata: Metadata,
    specs: Specs,
}

impl ClickhouseUtil {
    pub fn new(client: Client) -> Self {
        Self::build(false, client)
    }

    pub fn with_debug(client: Client) -> Self {
        Self::build(true, client)
    }

    fn build(dbg: bool, client: Client) -> Self {
        Self {
            dbg,
            client,
            metadata: Metadata::new(),
            specs: Specs::new(),
        }
    }
}

impl ClickhouseUtil {
    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ClickhouseUtil]: {}", s);
        }
    }
}
