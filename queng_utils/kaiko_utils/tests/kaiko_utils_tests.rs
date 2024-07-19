use docker_utils::DockerUtil;
use kaiko_utils::prelude::{KaikoUtil, KaikoUtilError};
use specs_utils::prelude::api_proxy_container_specs;
use std::env;

// Starts a kaiko api proxy on localhost port 7777
async fn setup_ci_env() {
    // Set the environment variable.
    env::set_var("ENV", "CI");

    // Create new DockerUtil
    let ci_env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Initiate CI container
    let container_config = api_proxy_container_specs();
    ci_env
        .setup_container(&container_config)
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

    assert!(assets.len() > 0);

    let exchanges = kaiko_util
        .get_exchanges()
        .await
        .expect("Failed to get exchanges");

    assert!(exchanges.len() > 0);

    let instruments = kaiko_util
        .get_instruments()
        .await
        .expect("Failed to get instruments");

    assert!(instruments.len() > 0);

    let result = kaiko_util.get_stats().await;
    assert!(result.is_ok());

    if let Ok(stats) = result {
        assert!(stats.number_assets() > 0);
        assert!(stats.number_exchanges() > 0);
        assert!(stats.number_instruments() > 0);
    }
}
