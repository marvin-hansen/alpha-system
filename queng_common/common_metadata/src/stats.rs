use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaStats {
    download_timestamp: String,
    hash: String,
    number_assets: u32,
    number_exchanges: u32,
    number_instruments: u32,
}

impl MetaStats {
    pub fn new(
        download_timestamp: String,
        hash: String,
        number_assets: u32,
        number_exchanges: u32,
        number_instruments: u32,
    ) -> Self {
        Self {
            download_timestamp,
            hash,
            number_assets,
            number_exchanges,
            number_instruments,
        }
    }
}

impl MetaStats {
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

impl Display for MetaStats {
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
