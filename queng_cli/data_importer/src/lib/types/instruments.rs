use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsRoot {
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
    pub trade_end_time: Option<String>,
    #[serde(rename = "exchange_code")]
    exchange_code: String,
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
    pub metadata: Option<Metadata>,
    #[serde(rename = "trade_start_timestamp")]
    pub trade_start_timestamp: Option<u64>,
    #[serde(rename = "trade_end_timestamp")]
    pub trade_end_timestamp: Option<i64>,
    #[serde(rename = "trade_compressed_size")]
    pub trade_compressed_size: u64,
    #[serde(rename = "trade_count")]
    pub trade_count: u64,
}

impl Instrument {
    pub fn exchange_code(&self) -> &str {
        &self.exchange_code
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "pair_figi")]
    pub pair_figi: Option<String>,
    #[serde(rename = "instrument_figi")]
    pub instrument_figi: Option<String>,
}
