use crate::fields::{CDN_PROXY_URL, CI_PROXY_URL, CLUSTER_PROXY_URL, LOCAL_PROXY_URL};
use crate::init::InitManager;
use crate::utils::util_client;
use common_env::EnvironmentType;
use common_errors::InitError;
use common_metadata::{MetaDataSet, MetaStats};
use environment_manager::EnvironmentManager;
use reqwest::Client;

mod fields;
mod init;
mod utils;

/// Downloads the metadata statistics set from Kaiko.
///
/// If `auto_detect_proxy` is `true`, then the proxy is automatically detected.
/// Order of proxied download sources:
/// 1) proxy cdn on Cloudflare. if that is not reachable;
/// 2) proxy service on localhost. if that is not reachable;
///
/// If `auto_detect_proxy` is `false`, then a panic is thrown
/// because stats can only be downloaded with proxy enabled
///
/// # Errors
///
/// Returns an error if the metadata statistics set can't be downloaded.
///
pub async fn download_meta_data_stats(
    dbg: bool,
    auto_detect_proxy: bool,
) -> Result<MetaStats, InitError> {
    let im = if auto_detect_proxy {
        // When auto_detect_proxy is true,
        // detect the proxy and return the proxy init manager
        init_manager_with_proxy(dbg).await
    } else {
        // // When auto_detect_proxy is false, panic because stats can only be downloaded with proxy enabled
        panic!("Stats can only be downloaded with proxy enabled.")
    };

    match im.get_meta_data_stats().await {
        Ok(meta_stats) => Ok(meta_stats),
        Err(e) => Err(e),
    }
}

/// Downloads the metadata set from Kaiko.
///
/// If `auto_detect_proxy` is `true`, then the proxy is automatically detected.
/// Order of proxied download sources:
/// 1) proxy cdn on Cloudflare. if that is not reachable;
/// 2) proxy service on localhost. if that is not reachable;
/// 3) no proxy; use the kaiko API
///
///If `auto_detect_proxy` is `false`, then no proxy is used
/// and only the  kaiko API is used to download the metadata set.
/// Note, this is last resort since the CDN or a local proxy is
/// significantly faster than the kaiko API.
///
/// # Errors
///
/// Returns an error if the metadata set can't be downloaded.
///
pub async fn download_meta_data(
    dbg: bool,
    auto_detect_proxy: bool,
) -> Result<MetaDataSet, InitError> {
    let im = if auto_detect_proxy {
        // When auto_detect_proxy is true,
        // detect the proxy and return the proxy init manager
        init_manager_with_proxy(dbg).await
    } else {
        // When auto_detect_proxy is false,
        // return the non-proxy init manager
        InitManager::new(dbg)
    };

    // Regardless of auto_detect_proxy, run the init process and return the downloaded meta_data_set
    let meta_data = im.init().await;

    // drop the init manager and all temporary allocations with it.
    // Usually this happens implicitly with the default allocator,
    // but MiMalloc isn't always getting it.
    drop(im);

    // Return the final meta_data_set
    meta_data
}

/// Asynchronously returns an `InitManager` instance configured to use a proxy for downloading the metadata set.
///
/// The proxies are checked in the following order:
///
/// 1. The CDN proxy.
/// 2. The local proxy.
/// 3. The cluster proxy (if environment is `CLUSTER`).
/// 4. The CI proxy (if environment is `CI`).
///
/// If no proxy is detected, returns a non-proxy `InitManager` instance.
///
/// # Arguments
///
/// * `dbg` - A boolean indicating whether debug mode is enabled.
///
/// # Returns
///
/// A `Result` containing an `InitManager` instance, or an error if the proxy detection failed.
///
async fn init_manager_with_proxy(dbg: bool) -> InitManager {
    match detect_proxy().await {
        Some(proxy) => InitManager::with_proxy_url(dbg, proxy),
        // It is possible that no proxy was detected,
        // in this case, return the non-proxy init manager
        None => InitManager::new(dbg),
    }
}

/// Detects a proxy to use for downloading the metadata set.
///
/// The proxies are checked in the following order:
///
/// 1. The CDN proxy.
/// 2. The local proxy.
/// 3. The cluster proxy (if environment is `CLUSTER`).
/// 4. The CI proxy (if environment is `CI`).
///
/// Returns `None` if no proxy is available.
///
async fn detect_proxy() -> Option<&'static str> {
    let config_manager = EnvironmentManager::new();
    let client = &util_client::get_client();

    match config_manager.env_type() {
        EnvironmentType::UNKNOWN => {
            if test_proxy(client, CDN_PROXY_URL).await {
                return Some(CDN_PROXY_URL);
            }

            if test_proxy(client, LOCAL_PROXY_URL).await {
                return Some(LOCAL_PROXY_URL);
            }

            if test_proxy(client, CLUSTER_PROXY_URL).await {
                return Some(CLUSTER_PROXY_URL);
            }

            None
        }
        EnvironmentType::LOCAL => {
            if test_proxy(client, CDN_PROXY_URL).await {
                return Some(CDN_PROXY_URL);
            }

            if test_proxy(client, LOCAL_PROXY_URL).await {
                return Some(LOCAL_PROXY_URL);
            }

            None
        }
        EnvironmentType::CLUSTER => {
            if test_proxy(client, CDN_PROXY_URL).await {
                return Some(CDN_PROXY_URL);
            }

            if test_proxy(client, CLUSTER_PROXY_URL).await {
                return Some(CLUSTER_PROXY_URL);
            }

            None
        }
        EnvironmentType::CI => {
            if test_proxy(client, CDN_PROXY_URL).await {
                return Some(CDN_PROXY_URL);
            }

            if test_proxy(client, CI_PROXY_URL).await {
                return Some(CI_PROXY_URL);
            }

            None
        }
    }
}

/// Tests if a given proxy is reachable.
///
/// # Arguments
///
/// * `client` - a reqwest client
/// * `proxy_url` - the URL of the proxy to be tested
///
/// # Returns
///
/// `true` if the proxy is reachable, `false` otherwise.
///
async fn test_proxy(client: &Client, proxy_url: &str) -> bool {
    let url = format!("{}{}", proxy_url, "health");

    let resp = match client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(_) => return false,
    };

    resp.status().is_success()
}
