pub mod error;

use crate::error::KaikoClientError;
use anyhow::Result;
use common::prelude::{AssetRoot, ExchangesRoot, InstrumentsRoot};
use rest_client::RestClient;

const API_URL: &str = "https://reference-data-api.kaiko.io/v1/";

pub struct KaikoClient {
    client: RestClient,
}

impl KaikoClient {
    pub fn new() -> Result<Self, KaikoClientError> {
        let client = RestClient::new(API_URL.to_string()).expect("Failed to construct KaikoClient");

        Ok(KaikoClient { client })
    }
}

impl KaikoClient {
    /// Returns a list of supported assets.
    /// See <https://docs.kaiko.com/#assets>
    pub async fn get_assets(&self) -> Result<AssetRoot, KaikoClientError> {
        let result: Result<AssetRoot> = self.client.get("assets", None).await;
        match result {
            Ok(assets) => Ok(assets),
            Err(e) => Err(KaikoClientError::new(&format!(
                "Error retrieving assets: {}",
                e
            ))),
        }
    }

    /// Returns a list of supported exchanges.
    /// See <https://docs.kaiko.com/#exchanges>
    pub async fn get_exchanges(&self) -> std::result::Result<ExchangesRoot, KaikoClientError> {
        let result: Result<ExchangesRoot> = self.client.get("exchanges", None).await;
        match result {
            Ok(exchanges) => Ok(exchanges),
            Err(e) => Err(KaikoClientError::new(&format!(
                "Error retrieving exchanges: {}",
                e
            ))),
        }
    }

    /// Returns a list of supported instruments. There are three possible cases regarding the trading period:
    ///
    /// * Trading has started and new trading activity is still being consumed (start time fields will be set, but end time fields will be null)
    /// * Trading has been recorded but no recent activity has been seen (both start and end time fields are set)
    /// * No trading has been registered yet (both start and end time fields are null)
    ///
    /// See: <https://docs.kaiko.com/#instruments>
    ///
    pub async fn get_instruments(&self) -> Result<InstrumentsRoot, KaikoClientError> {
        let result: Result<InstrumentsRoot> = self.client.get("instruments", None).await;
        match result {
            Ok(instruments) => Ok(instruments),
            Err(e) => Err(KaikoClientError::new(&format!(
                "Error retrieving instruments: {}",
                e
            ))),
        }
    }
}
