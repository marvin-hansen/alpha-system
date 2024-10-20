use crate::init::InitManager;
use common_errors::prelude::InitError;
use common_metadata::prelude::MetaDataSet;

mod fields;
mod init;
mod utils;

pub async fn download_meta_data(
    dbg: bool,
    proxy_url: Option<&str>,
) -> Result<MetaDataSet, InitError> {
    let im = if proxy_url.is_some() {
        let proxy_url = proxy_url.expect("Failed to unwrap proxy URL");
        assert!(!proxy_url.is_empty(), "Proxy URL must not be empty");

        // Test if trailing slash exists; If not, panic.
        let last = proxy_url
            .chars()
            .last()
            .expect("Failed to extract last character from proxy URL");
        if last != '/' {
            panic!("Proxy URL must end with a trailing slash");
        }

        InitManager::with_proxy_url(dbg, proxy_url)
    } else {
        InitManager::new(dbg)
    };

    // let im = InitManager::new(dbg);
    match im.init().await {
        Ok(meta_data_set) => Ok(meta_data_set),
        Err(e) => Err(e),
    }
}
