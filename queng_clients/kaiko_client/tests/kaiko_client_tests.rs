use env_utils::EnvUtil;
use kaiko_client::error::KaikoClientError;
use kaiko_client::KaikoClient;
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

fn get_client() -> Result<KaikoClient, KaikoClientError> {
    let url: &str = "http://localhost:7777/";
    let kaiko_client = KaikoClient::with_url(url, false);
    assert!(kaiko_client.is_ok());

    Ok(kaiko_client.unwrap())
}

#[tokio::test]
async fn test_new() {
    let kaiko_client = KaikoClient::new();
    assert!(kaiko_client.is_ok());
}

#[tokio::test]
async fn test_get_assets() {
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

    assert_eq!(actual, expected)
}

#[tokio::test]
async fn test_get_exchanges() {
    setup_ci_env().await;

    let kaiko_client = get_client();
    assert!(kaiko_client.is_ok());

    let kaiko_client = kaiko_client.unwrap();

    let stats = kaiko_client.get_stats().await.expect("Failed to get stats");

    let result = kaiko_client.get_exchanges().await;
    assert!(result.is_ok());

    let exchanges = result.unwrap();

    let expected = stats.number_exchanges();
    let actual = exchanges.len() as u32;

    assert_eq!(actual, expected)
}

#[tokio::test]
async fn test_get_instruments() {
    setup_ci_env().await;

    let kaiko_client = get_client();
    assert!(kaiko_client.is_ok());

    let kaiko_client = kaiko_client.unwrap();

    let stats = kaiko_client.get_stats().await.expect("Failed to get stats");

    let result = kaiko_client.get_instruments().await;
    assert!(result.is_ok());

    let instruments = result.unwrap();

    let expected = stats.number_instruments();
    let actual = instruments.len() as u32;

    assert_eq!(actual, expected)
}
