use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
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

impl Stats {
    pub fn download_timestamp(&self) -> &str {
        &self.download_timestamp
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn number_assets(&self) -> u32 {
        self.number_assets
    }

    pub fn number_exchanges(&self) -> u32 {
        self.number_exchanges
    }

    pub fn number_instruments(&self) -> u32 {
        self.number_instruments
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Stats: download_timestamp: {}, hash: {}, number_assets: {} number_exchanges: {}, number_instruments: {}",
            self.download_timestamp,
            self.hash,
            self.number_assets,
            self.number_exchanges,
            self.number_instruments,
        )
    }
}
