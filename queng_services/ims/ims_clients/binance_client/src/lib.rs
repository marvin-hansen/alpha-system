// Unsafe code must be explicitly enabled to use it.
#[deny(unsafe_code)]
//
pub mod types;

use crate::types::BinanceExchangeInfo;
use anyhow::{bail, Result};
use rest_client::RestClient;
use std::fmt::Error;

const API_URL: &str = "https://api-gcp.binance.com/api/v3/";

pub struct BinanceRESTClient {
    client: RestClient,
}

impl BinanceRESTClient {
    pub fn new() -> Result<Self, Error> {
        let client = RestClient::new(API_URL.to_string())
            .expect("Failed to construct BinanceRESTClient client");

        Ok(BinanceRESTClient { client })
    }
}

impl BinanceRESTClient {
    async fn get_exchange_info(&self) -> Result<BinanceExchangeInfo> {
        let result: Result<BinanceExchangeInfo> = self.client.get("exchangeInfo", None).await;
        match result {
            Ok(exchange_info) => Ok(exchange_info),
            Err(e) => bail!(format!("Error retrieving channels: {:?}", e)),
        }
    }

    pub async fn get_available_symbols(&self) -> Result<Vec<String>> {
        let result: Result<BinanceExchangeInfo> = self.get_exchange_info().await;
        let exchange_info = match result {
            Ok(exchange_info) => exchange_info,
            Err(e) => bail!(format!("Error retrieving channels: {:?}", e)),
        };

        let symbols_list: Vec<String> = exchange_info
            .symbols
            .iter()
            .map(|f| f.symbol.clone())
            .collect();

        Ok(symbols_list)
    }
}
