use crate::model::asset::Asset;
use crate::prelude::UpdateAsset;
use common_metadata::prelude::MetaAsset;

impl Asset {
    pub fn from_meta_asset(meta_asset: MetaAsset) -> Self {
        Asset {
            asset_code: meta_asset.code.clone(),
            asset_hash: meta_asset.hash(),
            asset_name: meta_asset.name,
            asset_classes: meta_asset.asset_classes.into_iter().map(Some).collect(),
            asset_figi: meta_asset.metadata.and_then(|m| m.asset_figi),
        }
    }

    pub fn to_meta_asset(&self) -> MetaAsset {
        MetaAsset {
            code: self.asset_code.clone(),
            name: self.asset_name.clone(),
            asset_classes: self
                .asset_classes
                .iter()
                .filter_map(|c| c.clone())
                .collect(),
            asset_class: String::new(), // Assuming asset_class is not used
            metadata: None,
            addresses: None,
        }
    }
}

impl UpdateAsset {
    pub fn from_meta_asset(meta_asset: MetaAsset) -> Self {
        UpdateAsset {
            asset_name: meta_asset.name.clone(),
            asset_hash: meta_asset.hash(),
            asset_classes: meta_asset.asset_classes.into_iter().map(Some).collect(),
            asset_figi: meta_asset.metadata.and_then(|m| m.asset_figi),
        }
    }

    pub fn to_meta_asset(&self) -> MetaAsset {
        MetaAsset {
            code: String::new(), // Assuming code is not used in UpdateAsset
            name: self.asset_name.clone(),
            asset_classes: self
                .asset_classes
                .iter()
                .filter_map(|c| c.clone())
                .collect(),
            asset_class: String::new(), // Assuming asset_class is not used
            metadata: None,
            addresses: None,
        }
    }
}
