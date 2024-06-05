use crate::types::stats::Stats;
use common::prelude::{Asset, AssetRoot, Exchange, ExchangesRoot, Instrument, InstrumentsRoot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaDataSet {
    assets: AssetRoot,
    exchanges: ExchangesRoot,
    instruments: InstrumentsRoot,
    stats: Stats,
    hash: u64,
}

impl MetaDataSet {
    pub fn new(assets: Vec<Asset>, exchanges: Vec<Exchange>, instruments: Vec<Instrument>) -> Self {
        // The sum is used internally in the metadata to determine if something has changed.
        let sum = (assets.len() + exchanges.len() + instruments.len()) as u64;

        // The hash of the sum is used externally via the stats endpoint
        // to let downstream systems determine if something has changed.
        // Blake3 is one of the fastest hashes out there.
        // https://github.com/BLAKE3-team/BLAKE3
        let hash = blake3::hash(sum.to_string().as_ref());

        let stats = Stats::new(
            hash.to_string(),
            assets.len() as u32,
            exchanges.len() as u32,
            instruments.len() as u32,
        );

        Self {
            assets: AssetRoot {
                result: "OK".to_string(),
                data: assets,
            },
            exchanges: ExchangesRoot {
                result: "OK".to_string(),
                data: exchanges,
            },
            instruments: InstrumentsRoot {
                result: "OK".to_string(),
                data: instruments,
            },
            stats,
            hash: sum,
        }
    }
}

impl MetaDataSet {
    pub fn assets(&self) -> &AssetRoot {
        &self.assets
    }
    pub fn exchanges(&self) -> &ExchangesRoot {
        &self.exchanges
    }
    pub fn instruments(&self) -> &InstrumentsRoot {
        &self.instruments
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn hash(&self) -> u64 {
        self.hash
    }
}
