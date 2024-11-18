use common_metadata::MetaAsset;
use pg_mddb::prelude::Asset;

#[test]
fn test_from_meta_asset() {
    let meta_asset = MetaAsset {
        code: "test_code".to_string(),
        name: "test_name".to_string(),
        asset_class: "class1".to_string(),
        asset_classes: vec!["class1".to_string(), "class2".to_string()],
        metadata: None,
        addresses: None,
    };

    // Test conversion from MetaAsset to PostgresAsset
    let postgres_asset = Asset::from_meta_asset(meta_asset.clone());

    // Test conversion from PostgresAsset to MetaAsset
    let converted_meta_asset = postgres_asset.to_meta_asset();

    assert_eq!(meta_asset.code, converted_meta_asset.code);
    assert_eq!(meta_asset.name, converted_meta_asset.name);
    assert_eq!(meta_asset.asset_class, converted_meta_asset.asset_class);
    assert_eq!(meta_asset.asset_classes, converted_meta_asset.asset_classes);
    assert!(converted_meta_asset.metadata.is_none());
    assert!(converted_meta_asset.addresses.is_none());
}

#[test]
fn test_to_meta_asset() {
    let postgres_asset = Asset {
        asset_code: "test_code".to_string(),
        asset_hash: "25eb1ec88a9d31e7c5e61202ef0ab00d5873de90ff70dd80011a59bd0a5fbef6".to_string(),
        asset_name: "test_name".to_string(),
        asset_class: "class1".to_string(),
        asset_classes: vec![Some("class1".to_string()), Some("class2".to_string())],
        asset_figi: None,
    };

    let meta_asset = postgres_asset.to_meta_asset();

    assert_eq!(postgres_asset.asset_code, meta_asset.code);
    assert_eq!(postgres_asset.asset_name, meta_asset.name);
    assert_eq!(postgres_asset.asset_class, meta_asset.asset_class);
    assert_eq!(postgres_asset.asset_hash, meta_asset.hash());
    assert!(meta_asset.metadata.is_none());
    assert!(meta_asset.addresses.is_none());
}
