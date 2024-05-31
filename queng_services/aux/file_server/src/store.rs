use crate::types::meta_data_set::MetaDataSet;
use common::prelude::{AssetRoot, ExchangesRoot, InstrumentsRoot, SymbolMappingRoot};

// The _Root wrappers are required to preserver API compatibility
// with KAIKO and to preserve existing JSON serialization.

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Store {
    assets: AssetRoot,
    exchanges: ExchangesRoot,
    instruments: InstrumentsRoot,
    symbol_mapping: SymbolMappingRoot,
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
}
