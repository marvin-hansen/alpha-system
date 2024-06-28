use kaiko_client::error::KaikoClientError;
use kaiko_client::KaikoClient;

#[tokio::test]
async fn test_new() {
    let kaiko_client = KaikoClient::new();
    assert!(kaiko_client.is_ok());
}

#[tokio::test]
async fn test_with_local_proxy() {
    let kaiko_client = KaikoClient::new();
    assert!(kaiko_client.is_ok());
}

#[tokio::test]
async fn test_with_url() {
    let url: &str = "http://localhost:7777/";
    let kaiko_client = KaikoClient::with_url(url);
    assert!(kaiko_client.is_ok());
}

fn get_client() -> Result<KaikoClient, KaikoClientError> {
    let kaiko_client = KaikoClient::with_local_proxy();
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
        assert!(asset_root.data.len() > 0);
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
        assert!(exchanges_root.data.len() > 0);
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
        assert!(instruments_root.data.len() > 0);
    }
}

// Fails on CI unless Kaiko local proxy is running,
// #[tokio::test]
// async fn test_get_stats() {
//     let kaiko_client = get_client();
//     assert!(kaiko_client.is_ok());
//
//     let kaiko_client = kaiko_client.unwrap();
//
//     let result = kaiko_client.get_stats().await;
//     assert!(result.is_ok());
//
//     // Additional assertions can be performed based on the expected `AssetRoot` structure.
//     if let Ok(stats) = result {
//         // Perform further assertions on `instruments_root`
//         assert!(stats.number_assets() > 0);
//         assert!(stats.number_exchanges() > 0);
//         assert!(stats.number_instruments() > 0);
//     }
// }
