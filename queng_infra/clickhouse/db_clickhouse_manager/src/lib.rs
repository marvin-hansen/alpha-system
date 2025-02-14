/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

mod error;
mod query_gen;
mod query_ohlcv;
mod query_symbols;
mod query_trades;
mod stream_ohlcv;
mod stream_trades;
pub mod types;

use crate::error::ClickHouseDBError;
use common_database::ClickHouseConfig;
use klickhouse::{Client, ClientOptions};

const FN_NAME: &str = "[ClickhouseDBManager]:";

pub struct ClickhouseDBManager {
    client: Client,
}

impl ClickhouseDBManager {
    /// Creates a new `QueryDBManager` instance.
    ///
    /// # Arguments
    ///
    /// * `db_config: ClickHouseConfig` - The database configuration containing connection parameters.
    ///
    /// # Returns
    ///
    /// A new `QueryDBManager` instance connected to the database.
    ///
    /// # Errors
    ///
    /// Will return an error if the connection to the database fails.
    ///
    ///
    pub async fn new(db_config: ClickHouseConfig) -> Result<Self, ClickHouseDBError> {
        let destination = db_config.connection_string();
        let client = match Client::connect(destination.clone(), ClientOptions::default()).await {
            Ok(res) => res,
            Err(e) => {
                return Err(ClickHouseDBError::ConnectionFailed(format!(
                    "{FN_NAME} Failed to connect to {} due error {}",
                    &destination, e
                )));
            }
        };

        Ok(Self { client })
    }
}

impl ClickhouseDBManager {
    pub async fn is_open(&self) -> bool {
        !self.client.is_closed()
    }
}
