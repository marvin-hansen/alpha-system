use common_metadata::prelude::{AssetMetadata, MetaAsset};
use proto_mddb::proto::{
    CheckIfAssetIdExistsRequest, CheckIfAssetIdExistsResponse, CountAssetsRequest,
    CountAssetsResponse, GetAllAssetsRequest, GetAllAssetsResponse, GetAssetRequest,
    GetAssetResponse,
};

// Asset Request
pub fn get_count_assets_request() -> CountAssetsRequest {
    CountAssetsRequest {}
}

pub fn get_check_if_asset_exists_request(asset_id: &str) -> CheckIfAssetIdExistsRequest {
    CheckIfAssetIdExistsRequest {
        asset_id: asset_id.to_string(),
    }
}

pub fn get_assets_request(asset_id: &str) -> GetAssetRequest {
    GetAssetRequest {
        asset_id: asset_id.to_string(),
    }
}

pub fn get_all_assets_request() -> GetAllAssetsRequest {
    GetAllAssetsRequest {}
}

// Asset Response
pub fn get_count_assets_response(count: u64) -> CountAssetsResponse {
    CountAssetsResponse { count }
}

pub fn get_check_if_asset_exists_response(
    asset_id: &str,
    exists: bool,
) -> CheckIfAssetIdExistsResponse {
    CheckIfAssetIdExistsResponse {
        asset_id: asset_id.to_string(),
        exists,
    }
}

pub fn get_assets_response(asset_id: &str, asset: &MetaAsset) -> GetAssetResponse {
    GetAssetResponse {
        asset_id: asset_id.to_string(),
        asset: Option::from(meta_asset_to_proto_asset(asset)),
    }
}

pub fn get_all_assets_response(assets: Vec<MetaAsset>) -> GetAllAssetsResponse {
    GetAllAssetsResponse {
        assets: assets
            .iter()
            .map(|meta_asset: &MetaAsset| meta_asset_to_proto_asset(meta_asset))
            .collect::<Vec<proto_mddb::proto::ProtoMetaAsset>>(),
    }
}

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
