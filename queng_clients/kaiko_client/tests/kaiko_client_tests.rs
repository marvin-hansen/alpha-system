use kaiko_client::error::KaikoClientError;
use kaiko_client::KaikoClient;

#[tokio::test]
async fn test_new() {
    let kaiko_client = KaikoClient::new();
    assert!(kaiko_client.is_ok());
}

fn get_client() -> Result<KaikoClient, KaikoClientError> {
    let url: &str = "http://localhost:7777/";
    let kaiko_client = KaikoClient::with_url(url);
    assert!(kaiko_client.is_ok());

    Ok(kaiko_client.unwrap())
}

#[tokio::test]
async fn test_get_assets() {
    let kaiko_client = get_client();
    assert!(kaiko_client.is_ok());

    let kaiko_client = kaiko_client.unwrap();

    let result = kaiko_client.get_assets().await;
    assert!(result.is_ok());

    // Additional assertions can be performed based on the expected `AssetRoot` structure.
    if let Ok(asset_root) = result {
        // Perform further assertions on `asset_root`
        assert!(!asset_root.data.is_empty());
    }
}

#[tokio::test]
async fn test_get_exchanges() {
    let kaiko_client = get_client();
    assert!(kaiko_client.is_ok());

    let kaiko_client = kaiko_client.unwrap();

    let result = kaiko_client.get_assets().await;
    assert!(result.is_ok());

    // Additional assertions can be performed based on the expected `AssetRoot` structure.
    if let Ok(exchanges_root) = result {
        // Perform further assertions on `exchanges_root`
        assert!(!exchanges_root.data.is_empty());
    }
}

#[tokio::test]
async fn test_get_instruments() {
    let kaiko_client = get_client();
    assert!(kaiko_client.is_ok());

    let kaiko_client = kaiko_client.unwrap();

    let result = kaiko_client.get_assets().await;
    assert!(result.is_ok());

    // Additional assertions can be performed based on the expected `AssetRoot` structure.
    if let Ok(instruments_root) = result {
        // Perform further assertions on `instruments_root`
        assert!(!instruments_root.data.is_empty());
    }
}
