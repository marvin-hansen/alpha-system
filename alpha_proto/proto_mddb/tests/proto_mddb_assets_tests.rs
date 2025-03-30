/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use proto_mddb::proto::ProtoMetaAsset;
#[test]
fn test_proto_meta_asset() {
    let asset = ProtoMetaAsset {
        asset_code: "asset_code".to_string(),
        asset_name: "asset_name".to_string(),
        asset_class: "asset_class".to_string(),
        asset_classes: vec![],
        asset_figi: None,
        asset_hash: "asset_hash".to_string(),
    };

    assert_eq!(asset.asset_code, "asset_code");
    assert_eq!(asset.asset_name, "asset_name");
    assert_eq!(asset.asset_class, "asset_class");
    assert_eq!(asset.asset_classes.len(), 0);
    assert_eq!(asset.asset_figi, None);
    assert_eq!(asset.asset_hash, "asset_hash");
}
