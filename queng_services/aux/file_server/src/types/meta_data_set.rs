use crate::types::stats::Stats;
use common::prelude::{
    Asset, AssetRoot, Exchange, ExchangesRoot, Instrument, InstrumentsRoot, SymbolMapping,
    SymbolMappingRoot,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaDataSet {
    assets: AssetRoot,
    exchanges: ExchangesRoot,
    instruments: InstrumentsRoot,
    symbol_mapping: SymbolMappingRoot,
    stats: Stats,
    hash: u64,
}

impl MetaDataSet {
    pub fn new(
        assets: Vec<Asset>,
        exchanges: Vec<Exchange>,
        instruments: Vec<Instrument>,
        symbol_mapping: BTreeMap<String, SymbolMapping>,
    ) -> Self {
        let stats = Stats::new(
            assets.len() as u32,
            exchanges.len() as u32,
            instruments.len() as u32,
            symbol_mapping.len() as u32,
        );

        let hash =
            (assets.len() + exchanges.len() + instruments.len() + symbol_mapping.len()) as u64;

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
            symbol_mapping: SymbolMappingRoot {
                result: "OK".to_string(),
                data: symbol_mapping,
            },
            stats,
            hash,
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
    pub fn symbol_mapping(&self) -> &SymbolMappingRoot {
        &self.symbol_mapping
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    pub fn hash(&self) -> u64 {
        self.hash
    }
}
