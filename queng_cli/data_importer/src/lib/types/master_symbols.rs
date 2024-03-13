use klickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct MasterSymbolRow {
    data: Vec<String>,
}

impl MasterSymbolRow {
    pub fn data(&self) -> &Vec<String> {
        &self.data
    }
}

#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct MasterSymbol {
    master_symbol: String,
    asset_class: String,
    base_asset: String,
    quote_asset: String,
}
