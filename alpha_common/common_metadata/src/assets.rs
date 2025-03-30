/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaAssetRoot {
    pub result: String,
    pub data: Vec<MetaAsset>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl Display for MetaAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MetaAsset: {self:?}")
    }
}

impl MetaAsset {
    #[must_use]
    pub fn hash(&self) -> String {
        // Metadata and addresses are not stored in the DB, thus not included in the hash.
        let binding = format!(
            "{}{}{}{}",
            self.code,
            self.name,
            self.asset_class,
            self.asset_classes.join(",")
        );
        let input = binding.as_bytes();
        let hash = blake3::hash(input);
        hash.to_string()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl Display for AssetMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AssetMetadata: {self:?}")
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub address: String,
    pub blockchain: String,
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Address: {self:?}")
    }
}
