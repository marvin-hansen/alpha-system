use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolMappingRoot {
    // Symbol code <==> Symbol Mapping
    data: HashMap<String, SymbolMapping>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolMapping {
    symbol_code: String,
    symbol_class: String,
    base_asset: String,
    quote_asset: String,
    // Exchange code <==> Exchange pair code
    instrument_mapping: HashMap<String, String>,
}

impl SymbolMapping {
    pub fn new(
        symbol_code: String,
        symbol_class: String,
        base_asset: String,
        quote_asset: String,
        instrument_mapping: HashMap<String, String>,
    ) -> Self {
        Self {
            symbol_code,
            symbol_class,
            base_asset,
            quote_asset,
            instrument_mapping,
        }
    }
}

impl SymbolMapping {
    pub fn symbol_code(&self) -> &str {
        &self.symbol_code
    }
    pub fn symbol_class(&self) -> &str {
        &self.symbol_class
    }
    pub fn base_asset(&self) -> &str {
        &self.base_asset
    }
    pub fn quote_asset(&self) -> &str {
        &self.quote_asset
    }
    pub fn instrument_mapping(&self) -> &HashMap<String, String> {
        &self.instrument_mapping
    }
}
