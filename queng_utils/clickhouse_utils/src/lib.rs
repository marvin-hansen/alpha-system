mod db;

pub mod error;
pub(crate) mod fields;
pub(crate) mod query;
pub mod setup;
pub mod teardown;
pub mod types;
pub(crate) mod utils;

use crate::db::metadata::Metadata;
use crate::db::specs::Specs;
use klickhouse::{Client, ClientOptions};

pub struct ClickhouseUtil {
    dbg: bool,
    client: Client,
    metadata: Metadata,
    specs: Specs,
}

impl ClickhouseUtil {
    pub async fn new(dsn: String) -> Self {
        Self::build(false, Self::get_clickhouse_client(dsn).await)
    }

    pub async fn with_debug(dsn: String) -> Self {
        Self::build(true, Self::get_clickhouse_client(dsn).await)
    }

    pub fn from_client(client: Client) -> Self {
        Self::build(false, client)
    }

    pub fn from_client_with_debug(client: Client) -> Self {
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

    pub async fn get_clickhouse_client(dsn: String) -> Client {
        let client = Client::connect(dsn.clone(), ClientOptions::default())
            .await
            .expect(format!("[get_clickhouse_client]: Failed to connect to {}", &dsn).as_str());

        client
    }
}

impl ClickhouseUtil {
    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ClickhouseUtil]: {}", s);
        }
    }
}
