use crate::types::symbol::BinanceSymbol;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct BinanceExchangeInfo {
    #[serde(rename = "timezone")]
    pub timezone: String,
    #[serde(rename = "serverTime")]
    pub server_time: u64,
    #[serde(rename = "symbols")]
    pub symbols: Vec<BinanceSymbol>,
}
