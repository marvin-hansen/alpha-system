use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    download_timestamp: String,
    hash: String,
    number_assets: u32,
    number_exchanges: u32,
    number_instruments: u32,
}

impl Stats {
    pub fn new(
        hash: String,
        number_assets: u32,
        number_exchanges: u32,
        number_instruments: u32,
    ) -> Self {
        Self {
            download_timestamp: Utc::now().to_rfc2822(),
            hash,
            number_assets,
            number_exchanges,
            number_instruments,
        }
    }
}
