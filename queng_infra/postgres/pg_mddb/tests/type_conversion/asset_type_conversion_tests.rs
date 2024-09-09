use common_metadata::prelude::MetaAsset;
use pg_metadb::prelude::Asset;

#[test]
fn test_asset_conversion() {
    let meta_asset = MetaAsset {
        code: "test_code".to_string(),
        name: "test_name".to_string(),
        asset_classes: vec!["class1".to_string(), "class2".to_string()],
        asset_class: "class1".to_string(),
        metadata: None,
        addresses: None,
    };

    // Test conversion from MetaAsset to PostgresAsset
    let postgres_asset = Asset::from_meta_asset(meta_asset.clone());

    // Test conversion from PostgresAsset to MetaAsset
    let converted_meta_asset = postgres_asset.to_meta_asset();

    assert_eq!(meta_asset.code, converted_meta_asset.code);
    assert_eq!(meta_asset.name, converted_meta_asset.name);
    assert_eq!(meta_asset.asset_classes, converted_meta_asset.asset_classes);
    assert!(converted_meta_asset.metadata.is_none());
    assert!(converted_meta_asset.addresses.is_none());
}
