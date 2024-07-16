mod query_gen;
mod query_ohlcv;
mod query_symbols;
mod query_trades;
mod stream_ohlcv;
mod stream_trades;
pub mod types;

use common_database::prelude::ClickHouseConfig;
use klickhouse::{Client, ClientOptions};
use std::fmt::Error;

const FN_NAME: &str = "[QueryDBManager]:";

pub struct ClickhouseDBManager {
    client: Client,
}

impl ClickhouseDBManager {
    /// Creates a new QueryDBManager instance.
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
    ///
    pub async fn new(db_config: ClickHouseConfig) -> Result<Self, Error> {
        let destination = db_config.connection_string();
        let client = Client::connect(destination.clone(), ClientOptions::default())
            .await
            .unwrap_or_else(|_| panic!("{} Failed to connect to {}", FN_NAME, &destination));

        Ok(Self { client })
    }
}

impl ClickhouseDBManager {
    pub async fn is_open(&self) -> bool {
        !self.client.is_closed()
    }
}
