use kaiko_utils::KaikoUtil;

// Requires a running kaiko api proxy on localhost port 7777

#[tokio::test]
async fn test_with_debug() {
    let kaiko_util = KaikoUtil::with_debug();
    assert!(kaiko_util.is_ok())
}

#[tokio::test]
async fn test_get_assets() {
    let kaiko_util = KaikoUtil::with_debug();
    assert!(kaiko_util.is_ok());

    let kaiko_util = kaiko_util.unwrap();
    let assets = kaiko_util.get_assets().await.expect("Failed to get assets");

    assert!(assets.len() > 0)
}

#[tokio::test]
async fn test_get_exchanges() {
    let kaiko_util = KaikoUtil::with_debug();
    assert!(kaiko_util.is_ok());

    let kaiko_util = kaiko_util.unwrap();

    let exchanges = kaiko_util
        .get_exchanges()
        .await
        .expect("Failed to get exchanges");

    assert!(exchanges.len() > 0)
}

#[tokio::test]
async fn test_get_instruments() {
    let kaiko_util = KaikoUtil::with_debug();
    assert!(kaiko_util.is_ok());

    let kaiko_util = kaiko_util.unwrap();
    let instruments = kaiko_util
        .get_instruments()
        .await
        .expect("Failed to get instruments");

    assert!(instruments.len() > 0)
}

#[tokio::test]
async fn test_get_stats() {
    let kaiko_util = KaikoUtil::with_debug();
    assert!(kaiko_util.is_ok());

    let kaiko_util = kaiko_util.unwrap();
    let result = kaiko_util.get_stats().await;
    assert!(result.is_ok());

    if let Ok(stats) = result {
        assert!(stats.number_assets() > 0);
        assert!(stats.number_exchanges() > 0);
        assert!(stats.number_instruments() > 0);
    }
}
