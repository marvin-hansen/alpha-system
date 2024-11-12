use crate::assets_utils::meta_asset_to_proto_asset;
use common_metadata::prelude::MetaAsset;
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

pub fn get_asset_request(asset_id: &str) -> GetAssetRequest {
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

pub fn get_check_if_asset_exists_response(exists: bool) -> CheckIfAssetIdExistsResponse {
    CheckIfAssetIdExistsResponse { exists }
}

pub fn get_assets_response(meta_asset: Option<MetaAsset>) -> GetAssetResponse {
    if let Some(assets) = meta_asset {
        GetAssetResponse {
            asset: Option::from(meta_asset_to_proto_asset(&assets)),
        }
    } else {
        GetAssetResponse { asset: None }
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
