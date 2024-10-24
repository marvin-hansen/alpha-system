use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaInstrumentsRoot {
    pub result: String,
    pub data: Vec<MetaInstrument>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaInstrument {
    #[serde(rename = "kaiko_legacy_exchange_slug")]
    pub kaiko_legacy_exchange_slug: String,
    #[serde(rename = "trade_start_time")]
    pub trade_start_time: Option<String>,
    #[serde(rename = "trade_end_time")]
    pub trade_end_time: Option<String>,
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
    pub metadata: Option<InstrumentMetadata>,
    #[serde(rename = "trade_start_timestamp")]
    pub trade_start_timestamp: Option<u64>,
    #[serde(rename = "trade_end_timestamp")]
    pub trade_end_timestamp: Option<i64>,
    #[serde(rename = "trade_compressed_size")]
    pub trade_compressed_size: u64,
    #[serde(rename = "trade_count")]
    pub trade_count: u64,
}

impl Display for MetaInstrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MetaInstrument: {:?}", self)
    }
}

impl MetaInstrument {
    pub fn exchange_code(&self) -> &str {
        &self.exchange_code
    }
}

impl MetaInstrument {
    // This is necessary because names are not unique across asset classes even on the same exchange.
    pub fn primary_key(&self) -> String {
        format!(
            "{}_{}_{}",
            self.exchange_code,
            self.class,
            self.exchange_pair_code.to_lowercase()
        )
    }

    pub fn hash(&self) -> String {
        let binding = self.to_string();
        let input = binding.as_bytes();
        let hash = blake3::hash(input);
        hash.to_string()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentMetadata {
    #[serde(rename = "pair_figi")]
    pub pair_figi: Option<String>,
    #[serde(rename = "instrument_figi")]
    pub instrument_figi: Option<String>,
}

impl Display for InstrumentMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstrumentMetadata: {:?}", self)
    }
}
