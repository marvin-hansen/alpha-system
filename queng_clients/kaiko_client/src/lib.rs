pub mod error;

use crate::error::KaikoClientError;
use anyhow::Result;
use common::prelude::{AssetRoot, ExchangesRoot, InstrumentsRoot, Stats};
use rest_client::{HeaderMap, HeaderValue, RestClient};

const API_URL: &str = "https://reference-data-api.kaiko.io/v1/";
const API_PROXY_URL: &str = "http://localhost:7777/";

pub struct KaikoClient {
    client: RestClient,
}

impl KaikoClient {
    pub fn new() -> Result<Self, KaikoClientError> {
        let client = Self::get_client(API_URL.to_string());

        Ok(KaikoClient { client })
    }

    pub fn with_local_proxy() -> Result<Self, KaikoClientError> {
        let client = Self::get_client(API_PROXY_URL.to_string());
        Ok(KaikoClient { client })
    }

    pub fn with_url(url: &str) -> Result<Self, KaikoClientError> {
        let client = Self::get_client(url.to_string());

        Ok(KaikoClient { client })
    }

    fn get_client(url: String) -> RestClient {
        let mut header_map = HeaderMap::new();

        header_map.insert("Accept", HeaderValue::from_static("application/json"));

        // Build client with headers

        RestClient::with_headers(url, header_map, true).expect("Failed to construct KaikoClient")
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

    pub async fn get_stats(&self) -> Result<Stats, KaikoClientError> {
        let result: Result<Stats> = self.client.get("stats", None).await;
        match result {
            Ok(stats) => Ok(stats),
            Err(e) => Err(KaikoClientError::new(&format!(
                "Error retrieving stats: {}",
                e
            ))),
        }
    }
}
