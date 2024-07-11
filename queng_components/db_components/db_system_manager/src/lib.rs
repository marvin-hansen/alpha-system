mod db_cfg;
mod db_svc;
mod gen_query;
mod types;

use crate::types::TestRow;
use clickhouse_utils::types::error::QueryError;
use common::prelude::{ClickHouseConfig, PortfolioConfig, ServiceConfig, ServiceID};
use klickhouse::{Client, ClientOptions};
use std::collections::HashMap;
use std::fmt::Error;
use std::sync::{Arc, RwLock};

const FN_NAME: &str = "[SystemDBManager]:";

// Interior mutability in Rust, part 2: thread safety
// https://ricardomartins.cc/2016/06/25/interior-mutability-thread-safety
type SafeRef<K, V> = Arc<RwLock<HashMap<K, V>>>;

#[derive(Clone)]
pub struct SystemDBManager {
    client: Client,
    portfolio_cache: SafeRef<u32, PortfolioConfig>,
    service_cache: SafeRef<ServiceID, ServiceConfig>,
}

impl SystemDBManager {
    /// Creates a new SystemDBManager instance.
    ///
    /// # Arguments
    ///
    /// * `db_config: ClickHouseConfig` - The database configuration containing connection parameters.
    ///
    /// # Returns
    ///
    /// A new QueryDBManager instance connected to the database.
    ///
    /// # Errors
    ///
    /// Will return an error if the connection to the database fails.
    ///
    /// # Example
    ///
    pub async fn new(db_config: &ClickHouseConfig) -> Result<Self, Error> {
        // Connect to the database
        let destination = db_config.connection_string();
        let client = Client::connect(destination.clone(), ClientOptions::default())
            .await
            .unwrap_or_else(|_| panic!("{} Failed to connect to {}", FN_NAME, &destination));

        // Initialize the cache
        let service_cache = Arc::new(RwLock::new(HashMap::new()));
        let portfolio_cache = Arc::new(RwLock::new(HashMap::new()));

        Ok(Self {
            client,
            portfolio_cache,
            service_cache,
        })
    }
}

impl SystemDBManager {
    pub async fn is_open(&self) -> bool {
        !self.client.is_closed()
    }
}

impl SystemDBManager {
    pub async fn init(&self) -> Result<(), QueryError> {
        self.init_portfolio_cache()
            .await
            .expect("Failed to init portfolio cache");

        self.init_service_cache()
            .await
            .expect("Failed to init service cache");

        Ok(())
    }
    async fn init_portfolio_cache(&self) -> Result<(), QueryError> {
        // Build the query
        let query = gen_query::get_all_portfolios_query();

        //
        // Fix type mapping from Rust to ClickHouse
        //

        // Execute query
        let result_rows = self
            .client
            .query_collect::<TestRow>(&query)
            .await
            .unwrap_or_else(|_| panic!("{} Failed to execute query: {}", FN_NAME, query));

        // Check for empty result
        if result_rows.is_empty() {
            return Ok(());
        }
        Ok(())
    }

    async fn init_service_cache(&self) -> Result<(), QueryError> {
        // Build the query
        let query = gen_query::get_all_services_query();

        // Execute query
        let result_rows = self
            .client
            .query_collect::<TestRow>(&query)
            .await
            .unwrap_or_else(|_| panic!("{} Failed to execute query: {}", FN_NAME, query));

        // Check for empty result
        if result_rows.is_empty() {
            return Ok(());
        }

        Ok(())
    }
}
