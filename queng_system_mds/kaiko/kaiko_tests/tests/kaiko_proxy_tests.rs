/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use container_specs_kaiko::api_proxy_container_config;
use docker_utils::DockerUtil;
use kaiko_client::KaikoClient;
use kaiko_client::error::KaikoClientError;

// Starts a kaiko api proxy on localhost port 7777
async fn setup_ci_env() {
    // Create new DockerUtil
    let ci_env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Initiate CI container
    let container_config = api_proxy_container_config();
    ci_env
        .setup_container(&container_config)
        .expect("Failed to setup ci api proxy container");
}

fn get_client() -> Result<KaikoClient, KaikoClientError> {
    let url: &str = "http://localhost:7777/";
    let kaiko_client = KaikoClient::with_url(url, false);
    assert!(kaiko_client.is_ok());
    let client = kaiko_client.unwrap();
    Ok(client)
}

#[tokio::test]
async fn test_kaiko_client() {
    // Setup CI environment
    setup_ci_env().await;

    let kaiko_client = get_client();
    assert!(kaiko_client.is_ok());

    let kaiko_client = kaiko_client.unwrap();

    let stats = kaiko_client.get_stats().await.expect("Failed to get stats");

    let result = kaiko_client.get_assets().await;
    assert!(result.is_ok());

    let assets = result.unwrap();

    let expected = stats.number_assets();
    let actual = assets.len() as u32;

    assert_eq!(actual, expected);

    let result = kaiko_client.get_exchanges().await;
    assert!(result.is_ok());

    let exchanges = result.unwrap();

    let expected = stats.number_exchanges();
    let actual = exchanges.len() as u32;

    assert_eq!(actual, expected);

    let result = kaiko_client.get_instruments().await;
    assert!(result.is_ok());

    let instruments = result.unwrap();

    let expected = stats.number_instruments();
    let actual = instruments.len() as u32;

    assert_eq!(actual, expected);
}
