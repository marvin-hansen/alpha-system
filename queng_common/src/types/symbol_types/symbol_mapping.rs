use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstrumentMapping {
    exchange_code: String,
    exchange_pair_code: String,
    instrument_figi: Option<String>,
    trade_count: u64,
}

impl InstrumentMapping {
    pub fn new(
        exchange_code: String,
        exchange_pair_code: String,
        instrument_figi: Option<String>,
        trade_count: u64,
    ) -> Self {
        Self {
            exchange_code,
            exchange_pair_code,
            instrument_figi,
            trade_count,
        }
    }

    pub fn exchange_code(&self) -> &str {
        &self.exchange_code
    }
    pub fn exchange_pair_code(&self) -> &str {
        &self.exchange_pair_code
    }
    pub fn instrument_figi(&self) -> &Option<String> {
        &self.instrument_figi
    }
    pub fn trade_count(&self) -> u64 {
        self.trade_count
    }
}

impl Display for InstrumentMapping {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "exchange_code: {}, \
            exchange_pair_code: {}, \
            instrument_figi: {:?}, \
            trade_count: {}",
            self.exchange_code, self.exchange_pair_code, self.instrument_figi, self.trade_count
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolMappingRoot {
    pub result: String,
    // Symbol code <==> Symbol Mapping
    pub data: BTreeMap<String, SymbolMapping>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolMapping {
    symbol_code: String,
    symbol_class: String,
    symbol_figi: Option<String>,
    base_asset: String,
    quote_asset: String,
    // Exchange code <==> Exchange pair code
    instrument_mapping: HashMap<String, InstrumentMapping>,
}

impl SymbolMapping {
    pub fn new(
        symbol_code: String,
        symbol_class: String,
        symbol_figi: Option<String>,
        base_asset: String,
        quote_asset: String,
        instrument_mapping: HashMap<String, InstrumentMapping>,
    ) -> Self {
        Self {
            symbol_code,
            symbol_class,
            symbol_figi,
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
    pub fn instrument_mapping(&self) -> &HashMap<String, InstrumentMapping> {
        &self.instrument_mapping
    }
}

impl Display for SymbolMapping {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "symbol_code: {}, symbol_class: {}, symbol_figi: {:?}, base_asset: {}, quote_asset: {}, instrument_mapping: {:?}",
            self.symbol_code,
            self.symbol_class,
            self.symbol_figi,
            self.base_asset,
            self.quote_asset,
            self.instrument_mapping,
        )
    }
}
