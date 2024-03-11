use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub result: String,
    pub data: Vec<Instrument>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    #[serde(rename = "kaiko_legacy_exchange_slug")]
    pub kaiko_legacy_exchange_slug: String,
    #[serde(rename = "trade_start_time")]
    pub trade_start_time: Option<String>,
    #[serde(rename = "trade_end_time")]
    pub trade_end_time: Value,
    #[serde(rename = "exchange_code")]
    pub exchange_code: String,
    #[serde(rename = "exchange_pair_code")]
    pub exchange_pair_code: String,
    #[serde(rename = "base_asset")]
    pub base_asset: String,
    #[serde(rename = "quote_asset")]
    pub quote_asset: String,
    #[serde(rename = "kaiko_legacy_symbol")]
    pub kaiko_legacy_symbol: String,
    pub code: String,
    pub class: String,
    pub metadata: Value,
    #[serde(rename = "trade_start_timestamp")]
    pub trade_start_timestamp: Option<i64>,
    #[serde(rename = "trade_end_timestamp")]
    pub trade_end_timestamp: Value,
    #[serde(rename = "trade_compressed_size")]
    pub trade_compressed_size: i64,
    #[serde(rename = "trade_count")]
    pub trade_count: i64,
}
