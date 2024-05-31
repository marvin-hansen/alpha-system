use common::prelude::{Asset, Exchange, Instrument, SymbolMapping};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaDataSet {
    assets: Vec<Asset>,
    exchanges: Vec<Exchange>,
    instruments: Vec<Instrument>,
    symbol_mapping: BTreeMap<String, SymbolMapping>,
}

impl MetaDataSet {
    pub fn new(
        assets: Vec<Asset>,
        exchanges: Vec<Exchange>,
        instruments: Vec<Instrument>,
        symbol_mapping: BTreeMap<String, SymbolMapping>,
    ) -> Self {
        Self {
            assets,
            exchanges,
            instruments,
            symbol_mapping,
        }
    }
}

impl MetaDataSet {
    pub fn assets(&self) -> &Vec<Asset> {
        &self.assets
    }
    pub fn exchanges(&self) -> &Vec<Exchange> {
        &self.exchanges
    }
    pub fn instruments(&self) -> &Vec<Instrument> {
        &self.instruments
    }

    pub fn symbol_mapping(&self) -> &BTreeMap<String, SymbolMapping> {
        &self.symbol_mapping
    }
}
