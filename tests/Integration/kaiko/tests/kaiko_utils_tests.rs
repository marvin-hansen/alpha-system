use env_utils::EnvUtil;
use kaiko_utils::KaikoUtil;
use kaiko_utils::KaikoUtilError;
use std::env;

// Starts a kaiko api proxy on localhost port 7777
async fn setup_ci_env() {
    // Set the environment variable.
    env::set_var("ENV", "CI");

    // Create new Env Utils
    let mut ci_env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    // Initiate CI container
    ci_env
        .setup_container_api_proxy()
        .await
        .expect("Failed to setup ci api proxy container");
}

fn get_client() -> Result<KaikoUtil, KaikoUtilError> {
    KaikoUtil::with_debug()
}

#[tokio::test]
async fn test_with_debug() {
    let kaiko_util = KaikoUtil::with_debug();
    assert!(kaiko_util.is_ok())
}

#[tokio::test]
async fn test_get_assets() {
    setup_ci_env().await;

    let kaiko_util = get_client();
    assert!(kaiko_util.is_ok());

    let kaiko_util = kaiko_util.unwrap();
    let assets = kaiko_util.get_assets().await.expect("Failed to get assets");

    assert!(assets.len() > 0)
}

#[tokio::test]
async fn test_get_exchanges() {
    setup_ci_env().await;

    let kaiko_util = get_client();
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
    setup_ci_env().await;

    let kaiko_util = get_client();
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
    setup_ci_env().await;

    let kaiko_util = get_client();
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
