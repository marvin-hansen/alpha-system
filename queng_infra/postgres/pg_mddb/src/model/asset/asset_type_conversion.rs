use crate::model::asset::Asset;
use crate::prelude::UpdateAsset;
use common_metadata::MetaAsset;

impl Asset {
    pub fn from_meta_asset(meta_asset: MetaAsset) -> Self {
        Asset {
            asset_code: meta_asset.code.clone(),
            asset_name: meta_asset.name.clone(),
            asset_class: meta_asset.asset_class.clone(),
            asset_classes: meta_asset
                .asset_classes
                .clone()
                .into_iter()
                .map(Some)
                .collect(),
            asset_figi: meta_asset.metadata.clone().and_then(|m| m.asset_figi),
            asset_hash: meta_asset.hash().clone(),
        }
    }

    pub fn to_meta_asset(&self) -> MetaAsset {
        MetaAsset {
            code: self.asset_code.clone(),
            name: self.asset_name.clone(),
            asset_class: self.asset_class.clone(),
            asset_classes: self
                .asset_classes
                .iter()
                .filter_map(|c| c.clone())
                .collect(),
            metadata: None,
            addresses: None,
        }
    }
}

impl UpdateAsset {
    pub fn from_meta_asset(meta_asset: MetaAsset) -> Self {
        UpdateAsset {
            asset_name: meta_asset.name.clone(),
            asset_class: meta_asset.asset_class.clone(),
            asset_classes: meta_asset
                .asset_classes
                .clone()
                .into_iter()
                .map(Some)
                .collect(),
            asset_figi: meta_asset.metadata.clone().and_then(|m| m.asset_figi),
            asset_hash: meta_asset.hash().clone(),
        }
    }

    pub fn to_meta_asset(&self) -> MetaAsset {
        MetaAsset {
            code: String::new(), // Assuming code is not used in UpdateAsset
            name: self.asset_name.clone(),
            asset_class: self.asset_class.clone(),
            asset_classes: self
                .asset_classes
                .iter()
                .filter_map(|c| c.clone())
                .collect(),
            metadata: None,
            addresses: None,
        }
    }
}
