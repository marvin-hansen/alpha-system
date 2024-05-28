use crate::types::meta_data_set::MetaDataSet;
use common::prelude::{Asset, Exchange, Instrument};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Store {
    assets: Vec<Asset>,
    exchanges: Vec<Exchange>,
    instruments: Vec<Instrument>,
}

impl Store {
    pub fn new(meta_data: MetaDataSet) -> Self {
        Self {
            assets: meta_data.assets().to_owned(),
            exchanges: meta_data.exchanges().to_owned(),
            instruments: meta_data.instruments().to_owned(),
        }
    }
}

impl Store {
    pub fn assets(&self) -> Vec<Asset> {
        self.assets.to_owned()
    }
    pub fn exchanges(&self) -> &Vec<Exchange> {
        &self.exchanges
    }
    pub fn instruments(&self) -> &Vec<Instrument> {
        &self.instruments
    }
}
