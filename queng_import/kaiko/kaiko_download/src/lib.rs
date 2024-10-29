use crate::fields::{CI_PROXY_URL, CLUSTER_PROXY_URL, LOCAL_PROXY_URL};
use crate::init::InitManager;
use crate::utils::util_client;
use common_env::prelude::EnvironmentType;
use common_errors::prelude::InitError;
use common_metadata::prelude::{MetaDataSet, MetaStats};
use environment_manager::EnvironmentManager;
use reqwest::Client;

mod fields;
mod init;
mod utils;

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
    match im.init().await {
        Ok(meta_data_set) => Ok(meta_data_set),
        Err(e) => Err(e),
    }
}

async fn init_manager_with_proxy(dbg: bool) -> InitManager {
    match detect_proxy().await {
        Some(proxy) => InitManager::with_proxy_url(dbg, proxy),
        // It is possible that no proxy was detected,
        // in this case, return the non-proxy init manager
        None => InitManager::new(dbg),
    }
}

async fn detect_proxy() -> Option<&'static str> {
    let config_manager = EnvironmentManager::new();
    let client = &util_client::get_client();

    match config_manager.env_type() {
        EnvironmentType::UNKNOWN => {
            if test_proxy(client, LOCAL_PROXY_URL).await {
                return Some(LOCAL_PROXY_URL);
            }

            if test_proxy(client, CLUSTER_PROXY_URL).await {
                return Some(CLUSTER_PROXY_URL);
            }

            None
        }
        EnvironmentType::LOCAL => {
            if test_proxy(client, LOCAL_PROXY_URL).await {
                return Some(LOCAL_PROXY_URL);
            }

            None
        }
        EnvironmentType::CLUSTER => {
            if test_proxy(client, CLUSTER_PROXY_URL).await {
                return Some(CLUSTER_PROXY_URL);
            }

            None
        }
        EnvironmentType::CI => {
            if test_proxy(client, CI_PROXY_URL).await {
                return Some(CI_PROXY_URL);
            }

            None
        }
    }
}

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
