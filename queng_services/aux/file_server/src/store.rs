use crate::types::meta_data_set::MetaDataSet;
use crate::types::stats::Stats;
use common::prelude::{AssetRoot, ExchangesRoot, InstrumentsRoot, SymbolMappingRoot};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type DB = Arc<RwLock<Store>>;

pub(crate) fn build_db(meta_data_set: MetaDataSet) -> DB {
    let store = Store::new(meta_data_set);
    Arc::new(RwLock::new(store))
}

// The _Root wrappers are required to preserver API compatibility
// with KAIKO and to preserve existing JSON serialization.

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Store {
    assets: AssetRoot,
    exchanges: ExchangesRoot,
    instruments: InstrumentsRoot,
    symbol_mapping: SymbolMappingRoot,
    stats: Stats,
}

impl Store {
    pub fn new(meta_data: MetaDataSet) -> Self {
        Self {
            assets: AssetRoot {
                result: "OK".to_string(),
                data: meta_data.assets().to_owned(),
            },
            exchanges: ExchangesRoot {
                result: "OK".to_string(),
                data: meta_data.exchanges().to_owned(),
            },
            instruments: InstrumentsRoot {
                result: "OK".to_string(),
                data: meta_data.instruments().to_owned(),
            },
            symbol_mapping: SymbolMappingRoot {
                result: "OK".to_string(),
                data: meta_data.symbol_mapping().to_owned(),
            },
            stats: meta_data.stats().to_owned(),
        }
    }
}

impl Store {
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
}
