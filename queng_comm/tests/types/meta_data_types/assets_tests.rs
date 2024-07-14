use common::prelude::{Address, AssetMetadata, MetaAsset, MetaAssetRoot};

#[test]
fn asset_structs_properties_test() {
    let metadata = AssetMetadata {
        eth_address: Some("0xETH".to_string()),
        bsc_address: Some("0xBSC".to_string()),
        polygon_address: Some("0xPOLY".to_string()),
        avalanche_address: Some("0xAVAX".to_string()),
        arbitrum_address: Some("0xARB".to_string()),
        ethereum_address: Some("0xETH_MAIN".to_string()),
        asset_figi: Some("FIGI123".to_string()),
    };

    let address = Address {
        address: "0xADDR123".to_string(),
        blockchain: "Ethereum".to_string(),
    };

    let asset = MetaAsset {
        code: "ASSET1".to_string(),
        name: "Test Asset".to_string(),
        asset_classes: vec!["Class1".to_string(), "Class2".to_string()],
        asset_class: "MainClass".to_string(),
        metadata: Some(metadata.clone()),
        addresses: Some(vec![address.clone()]),
    };

    let asset_root = MetaAssetRoot {
        result: "OK".to_string(),
        data: vec![asset.clone()],
    };

    // Test AssetRoot properties
    assert_eq!(asset_root.result, "OK");
    assert!(!asset_root.data.is_empty());

    // Test Asset properties
    let test_asset = &asset_root.data[0];
    assert_eq!(test_asset.code, "ASSET1");
    assert_eq!(test_asset.name, "Test Asset");
    assert_eq!(test_asset.asset_classes, vec!["Class1", "Class2"]);
    assert_eq!(test_asset.asset_class, "MainClass");

    // Test AssetMetadata properties
    let test_metadata = test_asset
        .metadata
        .as_ref()
        .expect("metadata should be present");
    assert_eq!(test_metadata.eth_address, Some("0xETH".to_string()));
    assert_eq!(test_metadata.bsc_address, Some("0xBSC".to_string()));
    assert_eq!(test_metadata.polygon_address, Some("0xPOLY".to_string()));
    assert_eq!(test_metadata.avalanche_address, Some("0xAVAX".to_string()));
    assert_eq!(test_metadata.arbitrum_address, Some("0xARB".to_string()));
    assert_eq!(
        test_metadata.ethereum_address,
        Some("0xETH_MAIN".to_string())
    );
    assert_eq!(test_metadata.asset_figi, Some("FIGI123".to_string()));

    // Test Address properties inside Asset
    let binding = test_asset.addresses.clone().unwrap();
    let test_address = binding.first().unwrap();
    assert_eq!(test_address.address, "0xADDR123");
    assert_eq!(test_address.blockchain, "Ethereum");
}
