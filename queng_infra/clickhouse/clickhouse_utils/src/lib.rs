mod db;

pub mod error;
pub mod types;

use crate::db::metadata::Metadata;
use klickhouse::{Client, ClientOptions};

pub use crate::db::utils;
pub use crate::error::ClickHouseQueryError;
pub use crate::error::ClickHouseUtilError;
pub use klickhouse::Client as ClickHouseClient;

pub struct ClickhouseUtil {
    dbg: bool,
    pub metadata: Metadata,
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
            metadata: Metadata::new(client, dbg),
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
