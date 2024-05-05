use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterSymbolRow {
    pub data: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterSymbol {
    pub master_symbol: String,
    pub asset_class: String,
    pub base_asset: String,
    pub quote_asset: String,
}
