use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaAssetRoot {
    pub result: String,
    pub data: Vec<MetaAsset>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaAsset {
    pub code: String,
    pub name: String,
    #[serde(rename = "asset_classes")]
    pub asset_classes: Vec<String>,
    #[serde(rename = "asset_class")]
    pub asset_class: String,
    pub metadata: Option<AssetMetadata>,
    #[serde(default)]
    pub addresses: Option<Vec<Address>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetMetadata {
    #[serde(rename = "eth_address")]
    pub eth_address: Option<String>,
    #[serde(rename = "bsc_address")]
    pub bsc_address: Option<String>,
    #[serde(rename = "polygon_address")]
    pub polygon_address: Option<String>,
    #[serde(rename = "avalanche_address")]
    pub avalanche_address: Option<String>,
    #[serde(rename = "arbitrum_address")]
    pub arbitrum_address: Option<String>,
    #[serde(rename = "ethereum_address")]
    pub ethereum_address: Option<String>,
    #[serde(rename = "asset_figi")]
    pub asset_figi: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub address: String,
    pub blockchain: String,
}
