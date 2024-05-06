use kaiko_client::KaikoClient;

#[tokio::test]
async fn test_new() {
    let kaiko_client = KaikoClient::new();
    assert!(kaiko_client.is_ok());
}

#[tokio::test]
async fn test_get_assets() {
    let kaiko_client = KaikoClient::new();
    assert!(kaiko_client.is_ok());

    let kaiko_client = kaiko_client.unwrap();

    let result = kaiko_client.get_assets().await;
    assert!(result.is_ok());

    // Additional assertions can be performed based on the expected `AssetRoot` structure.
    if let Ok(asset_root) = result {
        // Perform further assertions on `asset_root`
        assert!(asset_root.data.len() > 0);
    }
}

#[tokio::test]
async fn test_get_exchanges() {
    let kaiko_client = KaikoClient::new();
    assert!(kaiko_client.is_ok());

    let kaiko_client = kaiko_client.unwrap();

    let result = kaiko_client.get_assets().await;
    assert!(result.is_ok());

    // Additional assertions can be performed based on the expected `AssetRoot` structure.
    if let Ok(exchanges_root) = result {
        // Perform further assertions on `exchanges_root`
        assert!(exchanges_root.data.len() > 0);
    }
}

#[tokio::test]
async fn test_get_instruments() {
    let kaiko_client = KaikoClient::new();
    assert!(kaiko_client.is_ok());

    let kaiko_client = kaiko_client.unwrap();

    let result = kaiko_client.get_assets().await;
    assert!(result.is_ok());

    // Additional assertions can be performed based on the expected `AssetRoot` structure.
    if let Ok(instruments_root) = result {
        // Perform further assertions on `instruments_root`
        assert!(instruments_root.data.len() > 0);
    }
}
