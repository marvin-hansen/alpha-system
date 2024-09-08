use docker_utils::prelude::DockerUtil;
use kaiko_utils::prelude::{KaikoUtil, KaikoUtilError};
use std::env;

// Starts a kaiko api proxy on localhost port 7777
async fn setup_ci_env() {
    // Set the environment variable.
    env::set_var("ENV", "CI");
    // Create new DockerUtil
    let env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    env.setup_container_api_proxy()
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
async fn test_kaiko_utils() {
    setup_ci_env().await;

    let res = get_client();
    assert!(res.is_ok());
    let kaiko_util = res.unwrap();

    let assets = kaiko_util.get_assets().await.expect("Failed to get assets");

    assert!(!assets.is_empty());

    let exchanges = kaiko_util
        .get_exchanges()
        .await
        .expect("Failed to get exchanges");

    assert!(!exchanges.is_empty());

    let instruments = kaiko_util
        .get_instruments()
        .await
        .expect("Failed to get instruments");

    assert!(!instruments.is_empty());

    let result = kaiko_util.get_stats().await;
    assert!(result.is_ok());

    if let Ok(stats) = result {
        assert!(stats.number_assets() > 0);
        assert!(stats.number_exchanges() > 0);
        assert!(stats.number_instruments() > 0);
    }
}
