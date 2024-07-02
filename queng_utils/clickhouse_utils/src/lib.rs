mod db;

pub mod prelude;
pub mod query_utils;
pub mod setup;
pub mod teardown;
pub mod types;

use crate::db::metadata::Metadata;
use crate::db::specs::Specs;
use klickhouse::{Client, ClientOptions};
use types::error::ClickHouseUtilError;

pub struct ClickhouseUtil {
    dbg: bool,
    client: Client,
    //
    pub metadata: Metadata,
    pub specs: Specs,
}

impl ClickhouseUtil {
    pub async fn new(dsn: String) -> Result<Self, ClickHouseUtilError> {
        let client = Self::get_clickhouse_client(dsn)
            .await
            .expect("[ClickhouseUtil::new]: Failed to construct client");
        Self::build(false, client)
    }

    pub async fn with_debug(dsn: String) -> Result<Self, ClickHouseUtilError> {
        let client = Self::get_clickhouse_client(dsn)
            .await
            .expect("[ClickhouseUtil::with_debug]: Failed to construct client");
        Self::build(true, client)
    }

    pub fn from_client(client: Client) -> Result<Self, ClickHouseUtilError> {
        Self::build(false, client)
    }

    pub fn from_client_with_debug(client: Client) -> Result<Self, ClickHouseUtilError> {
        Self::build(true, client)
    }

    fn build(dbg: bool, client: Client) -> Result<Self, ClickHouseUtilError> {
        Ok(Self {
            dbg,
            client: client.clone(),
            metadata: Metadata::new(client.clone()),
            specs: Specs::new(client.clone()),
        })
    }

    async fn get_clickhouse_client(dsn: String) -> Result<Client, ClickHouseUtilError> {
        let client = Client::connect(dsn.clone(), ClientOptions::default())
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "[ClickhouseUtil::get_clickhouse_client]: Failed to connect to {}",
                    &dsn
                )
            });

        Ok(client)
    }
}

impl ClickhouseUtil {
    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ClickhouseUtil]: {}", s);
        }
    }
}
