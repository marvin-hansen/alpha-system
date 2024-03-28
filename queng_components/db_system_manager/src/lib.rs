mod db_cfg;
mod db_svc;

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
        let destination = db_config.connection_string();
        let client = Client::connect(destination.clone(), ClientOptions::default())
            .await
            .expect(format!("{} Failed to connect to {}", FN_NAME, &destination).as_str());

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
