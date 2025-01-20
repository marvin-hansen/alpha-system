pub mod error;

use crate::error::KaikoClientError;
use common_metadata::{
    MetaAsset, MetaAssetRoot, MetaExchange, MetaExchangesRoot, MetaInstrument, MetaInstrumentsRoot,
    MetaStats,
};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

const API_URL: &str = "https://reference-data-api.kaiko.io/v1/";
const API_PROXY_URL: &str = "http://localhost:7777/";

pub struct KaikoClient {
    client: Client,
    url: String,
}

impl KaikoClient {
    pub fn new() -> Result<Self, KaikoClientError> {
        // The origin API delivers large raw data so compression enabled by default is warranted
        let client = Self::get_client(true);

        Ok(Self {
            client,
            url: API_URL.to_string(),
        })
    }

    pub fn with_local_proxy() -> Result<Self, KaikoClientError> {
        let client = Self::get_client(true);
        Ok(Self {
            client,
            url: API_PROXY_URL.to_string(),
        })
    }

    pub fn with_url(url: &str, gzip: bool) -> Result<Self, KaikoClientError> {
        let client = Self::get_client(gzip);

        Ok(Self {
            client,
            url: url.to_string(),
        })
    }

    fn get_client(gzip: bool) -> Client {
        let mut header_map = HeaderMap::new();

        header_map.insert("Accept", HeaderValue::from_static("application/json"));

        // Enable gzip compressions for requests and responses to reduce download time.
        // https://dtantsur.github.io/rust-openstack/reqwest/struct.ClientBuilder.html

        reqwest::Client::builder()
            .default_headers(header_map)
            .gzip(gzip)
            .build()
            .expect("Failed to build reqwest client")
    }
}

impl KaikoClient {
    async fn download(&self, url: &str) -> Result<Vec<u8>, KaikoClientError> {
        let resp = self.client.get(url).send().await.expect("request failed");

        let body = resp.bytes().await.expect("body invalid");

        Ok(body.to_vec())
    }
}

impl KaikoClient {
    /// Returns a list of supported assets.
    /// See <https://docs.kaiko.com/#assets>
    pub async fn get_assets(&self) -> Result<Vec<MetaAsset>, KaikoClientError> {
        let url = format!("{}assets", self.url);
        return match self.download(&url).await {
            Ok(bytes) => {
                let assets_root: MetaAssetRoot =
                    serde_json::from_slice(bytes.as_slice()).expect("Failed to parse assets");

                Ok(assets_root.data)
            }
            Err(e) => Err(KaikoClientError::from(format!(
                "Error downloading assets {e}"
            ))),
        };
    }

    /// Returns a list of supported exchanges.
    /// See <https://docs.kaiko.com/#exchanges>
    pub async fn get_exchanges(&self) -> std::result::Result<Vec<MetaExchange>, KaikoClientError> {
        let url = format!("{}exchanges", self.url);
        return match self.download(&url).await {
            Ok(bytes) => {
                let exchanges_root: MetaExchangesRoot =
                    serde_json::from_slice(bytes.as_slice()).expect("Failed to parse exchanges");

                Ok(exchanges_root.data)
            }
            Err(e) => Err(KaikoClientError::from(format!(
                "Error downloading exchanges {e}"
            ))),
        };
    }

    /// Returns a list of supported instruments. There are three possible cases regarding the trading period:
    ///
    /// * Trading has started and new trading activity is still being consumed (start time fields will be set, but end time fields will be null)
    /// * Trading has been recorded but no recent activity has been seen (both start and end time fields are set)
    /// * No trading has been registered yet (both start and end time fields are null)
    ///
    /// See: <https://docs.kaiko.com/#instruments>
    ///
    pub async fn get_instruments(&self) -> Result<Vec<MetaInstrument>, KaikoClientError> {
        let url = format!("{}instruments", self.url);
        return match self.download(&url).await {
            Ok(bytes) => {
                let instruments_root: MetaInstrumentsRoot =
                    serde_json::from_slice(bytes.as_slice()).expect("Failed to parse exchanges");

                Ok(instruments_root.data)
            }
            Err(e) => Err(KaikoClientError::from(format!(
                "Error downloading instruments {e}"
            ))),
        };
    }

    pub async fn get_stats(&self) -> Result<MetaStats, KaikoClientError> {
        let url = format!("{}stats", self.url);

        return match self.download(&url).await {
            Ok(bytes) => {
                let stats: MetaStats =
                    serde_json::from_slice(bytes.as_slice()).expect("Failed to parse exchanges");

                Ok(stats)
            }
            Err(e) => Err(KaikoClientError::from(format!(
                "Error downloading stats {e}"
            ))),
        };
    }
}
