use common::prelude::{Asset, Exchange, Instrument};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaDataSet {
    assets: Vec<Asset>,
    exchanges: Vec<Exchange>,
    instruments: Vec<Instrument>,
}

impl MetaDataSet {
    pub fn new(assets: Vec<Asset>, exchanges: Vec<Exchange>, instruments: Vec<Instrument>) -> Self {
        Self {
            assets,
            exchanges,
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
    pub fn instruments(&self) -> &Vec<Instrument> {
        &self.instruments
    }
}
