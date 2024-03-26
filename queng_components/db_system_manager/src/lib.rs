mod db_cfg;
mod db_svc;

use common::prelude::ClickHouseConfig;
use klickhouse::{Client, ClientOptions};
use std::fmt::Error;

const FN_NAME: &str = "[SystemDBManager]:";

#[derive(Clone)]
pub struct SystemDBManager {
    client: Client,
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

        Ok(Self { client })
    }
}

impl SystemDBManager {
    pub async fn is_open(&self) -> bool {
        !self.client.is_closed()
    }
}
