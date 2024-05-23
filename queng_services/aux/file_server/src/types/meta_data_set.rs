use common::prelude::{Asset, Exchange, Instrument};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaDataSet {
    assets: Vec<Asset>,
    exchanges: Vec<Exchange>,
    exchanges_valid: Vec<String>,
    instruments: Vec<Instrument>,
}

impl MetaDataSet {
    pub fn new(
        assets: Vec<Asset>,
        exchanges: Vec<Exchange>,
        exchanges_valid: Vec<String>,
        instruments: Vec<Instrument>,
    ) -> Self {
        Self {
            assets,
            exchanges,
            exchanges_valid,
            instruments,
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
    pub fn exchanges_valid(&self) -> &Vec<String> {
        &self.exchanges_valid
    }
    pub fn instruments(&self) -> &Vec<Instrument> {
        &self.instruments
    }
}
