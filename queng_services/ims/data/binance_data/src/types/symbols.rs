use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ExchangeInfo {
    #[serde(rename = "timezone")]
    pub timezone: String,
    #[serde(rename = "serverTime")]
    pub server_time: u64,
    #[serde(rename = "symbols")]
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    // pub baseAsset: String,
    // pub baseAssetPrecision: u8,
    // pub quoteAsset: String,
    // pub quotePrecision: u8,
    // pub quoteAssetPrecision: u8,
    // pub baseCommissionPrecision: u8,
    // pub quoteCommissionPrecision: u8,
    // pub orderTypes: Vec<String>,
    // pub icebergAllowed: bool,
    // pub ocoAllowed: bool,
    // pub quoteOrderQtyMarketAllowed: bool,
    // pub isSpotTradingAllowed: bool,
    // pub isMarginTradingAllowed: bool,
}
