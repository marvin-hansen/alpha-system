mod db;

pub mod error;
mod import;
pub mod prelude;
mod query;
pub mod query_utils;
pub mod setup;
pub mod teardown;
pub mod types;

use crate::db::metadata::Metadata;
use crate::db::specs::Specs;
use klickhouse::{Client, ClientOptions, KlickhouseError};

// Re-export CH client
use crate::error::{ClickHouseUtilError, QueryError};
use crate::types::{CountRow, ExistsRow};
pub use klickhouse::Client as ClickHouseClient;

pub struct ClickhouseUtil {
    dbg: bool,
    client: Client,
    metadata: Metadata,
    specs: Specs,
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
            client,
            metadata: Metadata::new(),
            specs: Specs::new(),
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

impl ClickhouseUtil {
    /// Executes a query on the specified table in the ClickHouse database.
    pub(crate) async fn execute_query(&self, query: &str) -> Result<(), QueryError> {
        //
        self.dbg_print("[execute_query]: execute query");
        let res = self.client.execute(query).await;

        self.dbg_print("[execute_query]: check for errors");
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(QueryError::QueryFailed(e.to_string())),
        }
    }

    /// Queries whether the table exists. Returns true if so, otherwise false.
    pub(crate) async fn verify_table_exists(&self, query: &str) -> Result<bool, QueryError> {
        //
        self.dbg_print("[verify_table_exists]: execute table exists query");
        let res: Result<ExistsRow, KlickhouseError> = self.client.query_one(query).await;

        self.dbg_print("[verify_table_exists]: check for errors");
        match res {
            Ok(res) => Ok(res.exists()),
            Err(e) => Err(QueryError::QueryFailed(e.to_string())),
        }
    }

    /// Counts the number of rows in the specified table in the ClickHouse database.
    pub(crate) async fn count_rows(&self, table_name: &str) -> Result<u64, QueryError> {
        //
        self.dbg_print("Generate count query for the specified table");
        let count_query = format!("SELECT count(*) FROM {table_name}");

        // We need type annotation of the Result type here.
        self.dbg_print("Execute count query");
        let number_of_rows: Result<CountRow, KlickhouseError> =
            self.client.query_one(&count_query).await;

        match number_of_rows {
            Ok(number_of_rows) => Ok(number_of_rows.count()),
            Err(e) => Err(QueryError::QueryFailed(e.to_string())),
        }
    }
}
