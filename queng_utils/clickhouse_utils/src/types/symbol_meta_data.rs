use klickhouse::Row;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct SymbolMetaData {
    table_name: String,
    symbol: String,
    symbol_id: u32,
    number_of_rows: u64,
}

impl SymbolMetaData {
    pub fn new(table_name: String, symbol: String, symbol_id: u32, number_of_rows: u64) -> Self {
        Self {
            table_name,
            symbol,
            symbol_id,
            number_of_rows,
        }
    }
}

impl SymbolMetaData {
    pub fn table_name(&self) -> &str {
        &self.table_name
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn symbol_id(&self) -> u32 {
        self.symbol_id
    }
    pub fn number_of_rows(&self) -> u64 {
        self.number_of_rows
    }
}

impl fmt::Display for SymbolMetaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MetaData {{ table_name: {}, symbol: {}, symbol_id: {}, number_of_rows: {} }}",
            self.table_name, self.symbol, self.symbol_id, self.number_of_rows
        )
    }
}
