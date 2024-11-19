use common_metadata::{AssetMetadata, MetaAsset};

#[must_use]
pub fn meta_asset_to_proto_asset(meta_asset: &MetaAsset) -> proto_mddb::proto::ProtoMetaAsset {
    proto_mddb::proto::ProtoMetaAsset {
        asset_code: meta_asset.code.clone(),
        asset_name: meta_asset.name.clone(),
        asset_class: meta_asset.asset_class.clone(),
        asset_classes: meta_asset.asset_classes.clone(),
        asset_figi: if meta_asset.metadata.is_some() {
            meta_asset.metadata.clone().unwrap().asset_figi
        } else {
            None
        },
        asset_hash: meta_asset.hash(),
    }
}

#[must_use]
pub fn proto_asset_to_meta_asset(proto_asset: &proto_mddb::proto::ProtoMetaAsset) -> MetaAsset {
    MetaAsset {
        code: proto_asset.asset_code.clone(),
        name: proto_asset.asset_name.clone(),
        asset_class: proto_asset.asset_class.clone(),
        asset_classes: proto_asset.asset_classes.clone(),
        metadata: if proto_asset.asset_figi.is_some() {
            Some(AssetMetadata {
                eth_address: None,
                bsc_address: None,
                polygon_address: None,
                avalanche_address: None,
                arbitrum_address: None,
                ethereum_address: None,
                asset_figi: proto_asset.asset_figi.clone(),
            })
        } else {
            None
        },
        addresses: None,
    }
}
