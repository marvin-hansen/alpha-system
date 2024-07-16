mod db;

pub mod error;
pub mod prelude;
pub mod query_utils;
pub mod types;

use crate::db::metadata::Metadata;
use crate::db::specs::Specs;
use error::ClickHouseUtilError;
use klickhouse::{Client, ClientOptions};

pub struct ClickhouseUtil {
    dbg: bool,
    pub metadata: Metadata,
    pub specs: Specs,
}

impl ClickhouseUtil {
    pub async fn new(dsn: &str) -> Result<Self, ClickHouseUtilError> {
        Self::build(false, dsn).await
    }

    pub async fn with_debug(dsn: &str) -> Result<Self, ClickHouseUtilError> {
        Self::build(true, dsn).await
    }

    async fn build(dbg: bool, dsn: &str) -> Result<Self, ClickHouseUtilError> {
        if dbg {
            println!("[ClickhouseUtil]: Debug mode enabled");
        }

        let client = Self::get_clickhouse_client(dsn)
            .await
            .expect("[ClickhouseUtil::with_debug]: Failed to construct database client");

        Ok(Self {
            dbg,
            metadata: Metadata::new(client.clone(), dbg),
            specs: Specs::new(client.clone(), dbg),
        })
    }

    async fn get_clickhouse_client(dsn: &str) -> Result<Client, ClickHouseUtilError> {
        let client = Client::connect(dsn, ClientOptions::default())
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
