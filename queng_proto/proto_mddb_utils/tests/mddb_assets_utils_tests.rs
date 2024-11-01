use common_metadata::prelude::{AssetMetadata, MetaAsset};
use proto_mddb::proto::{
    CheckIfAssetIdExistsRequest, CheckIfAssetIdExistsResponse, CountAssetsRequest,
    GetAllAssetsRequest, GetAssetRequest, ProtoMetaAsset,
};
use proto_mddb_utils::mddb_assets_utils::*;

#[test]
fn test_get_count_assets_request() {
    let request = get_count_assets_request();
    assert_eq!(request, CountAssetsRequest {});
}

#[test]
fn test_get_check_if_asset_exists_request() {
    let asset_id = "test_asset_id";
    let request = get_check_if_asset_exists_request(asset_id);
    assert_eq!(
        request,
        CheckIfAssetIdExistsRequest {
            asset_id: asset_id.to_string()
        }
    );
}

#[test]
fn test_get_assets_request() {
    let asset_id = "test_asset_id";
    let request = get_assets_request(asset_id);
    assert_eq!(
        request,
        GetAssetRequest {
            asset_id: asset_id.to_string()
        }
    );
}

#[test]
fn test_get_all_assets_request() {
    let request = get_all_assets_request();
    assert_eq!(request, GetAllAssetsRequest {});
}

#[test]
fn test_get_count_assets_response() {
    let count = 42;
    let response = get_count_assets_response(count);
    assert_eq!(response, "Count: 42");
}

#[test]
fn test_get_check_if_asset_exists_response() {
    let asset_id = "test_asset_id";
    let exists = true;
    let response = get_check_if_asset_exists_response(asset_id, exists);
    assert_eq!(
        response,
        CheckIfAssetIdExistsResponse {
            asset_id: asset_id.to_string(),
            exists
        }
    );
}

#[test]
fn test_get_assets_response() {
    let asset_id = "test_asset_id";
    let meta_asset = MetaAsset {
        code: "code".to_string(),
        name: "name".to_string(),
        asset_class: "class".to_string(),
        asset_classes: vec!["class1".to_string(), "class2".to_string()],
        metadata: Some(AssetMetadata {
            eth_address: None,
            bsc_address: None,
            polygon_address: None,
            avalanche_address: None,
            arbitrum_address: None,
            ethereum_address: None,
            asset_figi: Some("figi".to_string()),
        }),
        addresses: None,
    };
    let response = get_assets_response(asset_id, &meta_asset);
    assert_eq!(response.asset_id, asset_id.to_string());
    assert!(response.asset.is_some());
}

#[test]
fn test_get_all_assets_response() {
    let meta_assets = vec![
        MetaAsset {
            code: "code1".to_string(),
            name: "name1".to_string(),
            asset_class: "class1".to_string(),
            asset_classes: vec!["class1".to_string()],
            metadata: None,
            addresses: None,
        },
        MetaAsset {
            code: "code2".to_string(),
            name: "name2".to_string(),
            asset_class: "class2".to_string(),
            asset_classes: vec!["class2".to_string()],
            metadata: None,
            addresses: None,
        },
    ];
    let response = get_all_assets_response(meta_assets.clone());
    assert_eq!(response.assets.len(), meta_assets.len());
}

#[test]
fn test_meta_asset_to_proto_asset() {
    let meta_asset = MetaAsset {
        code: "code".to_string(),
        name: "name".to_string(),
        asset_class: "class".to_string(),
        asset_classes: vec!["class1".to_string(), "class2".to_string()],
        metadata: Some(AssetMetadata {
            eth_address: None,
            bsc_address: None,
            polygon_address: None,
            avalanche_address: None,
            arbitrum_address: None,
            ethereum_address: None,
            asset_figi: Some("figi".to_string()),
        }),
        addresses: None,
    };
    let proto_asset = meta_asset_to_proto_asset(&meta_asset);
    assert_eq!(proto_asset.asset_code, meta_asset.code);
    assert_eq!(proto_asset.asset_name, meta_asset.name);
    assert_eq!(proto_asset.asset_class, meta_asset.asset_class);
    assert_eq!(proto_asset.asset_classes, meta_asset.asset_classes);
    assert_eq!(
        proto_asset.asset_figi,
        meta_asset.metadata.unwrap().asset_figi
    );
}

#[test]
fn test_proto_asset_to_meta_asset() {
    let proto_asset = ProtoMetaAsset {
        asset_code: "code".to_string(),
        asset_name: "name".to_string(),
        asset_class: "class".to_string(),
        asset_classes: vec!["class1".to_string(), "class2".to_string()],
        asset_figi: Some("figi".to_string()),
        asset_hash: "hash".to_string(),
    };
    let meta_asset = proto_asset_to_meta_asset(&proto_asset);
    assert_eq!(meta_asset.code, proto_asset.asset_code);
    assert_eq!(meta_asset.name, proto_asset.asset_name);
    assert_eq!(meta_asset.asset_class, proto_asset.asset_class);
    assert_eq!(meta_asset.asset_classes, proto_asset.asset_classes);
    assert_eq!(
        meta_asset.metadata.unwrap().asset_figi,
        proto_asset.asset_figi
    );
}
