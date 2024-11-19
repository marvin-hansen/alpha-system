use crate::{
    MetaAsset, MetaAssetRoot, MetaExchange, MetaExchangesRoot, MetaInstrument, MetaInstrumentsRoot,
    MetaStats,
};
use chrono::Utc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaDataSet {
    assets: MetaAssetRoot,
    exchanges: MetaExchangesRoot,
    instruments: MetaInstrumentsRoot,
    stats: MetaStats,
    hash: u64,
}

impl MetaDataSet {
    #[must_use]
    pub fn new(
        assets: Vec<MetaAsset>,
        exchanges: Vec<MetaExchange>,
        instruments: Vec<MetaInstrument>,
    ) -> Self {
        let sum = (assets.len() + exchanges.len() + instruments.len()) as u64;
        // The hash of the sum is used to determine if some meta-data have changed.
        let hash = crypto_utils::sha512_digest(sum.to_string());

        let download_timestamp = Utc::now().to_rfc2822();

        let stats = MetaStats::new(
            download_timestamp,
            hash,
            assets.len() as u32,
            exchanges.len() as u32,
            instruments.len() as u32,
        );

        Self {
            assets: MetaAssetRoot {
                result: "OK".to_string(),
                data: assets,
            },
            exchanges: MetaExchangesRoot {
                result: "OK".to_string(),
                data: exchanges,
            },
            instruments: MetaInstrumentsRoot {
                result: "OK".to_string(),
                data: instruments,
            },
            stats,
            hash: sum,
        }
    }
}

impl MetaDataSet {
    #[must_use]
    pub const fn assets(&self) -> &MetaAssetRoot {
        &self.assets
    }
    #[must_use]
    pub const fn exchanges(&self) -> &MetaExchangesRoot {
        &self.exchanges
    }
    #[must_use]
    pub const fn instruments(&self) -> &MetaInstrumentsRoot {
        &self.instruments
    }
    #[must_use]
    pub const fn stats(&self) -> &MetaStats {
        &self.stats
    }
    #[must_use]
    pub const fn hash(&self) -> u64 {
        self.hash
    }
}
