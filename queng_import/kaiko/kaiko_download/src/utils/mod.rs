use common_errors::prelude::DownloadError;
use reqwest::Client;

pub(crate) mod util_client;
pub(crate) mod util_download;

#[derive(Debug, Clone)]
pub struct DownloadUtils {
    client: Client,
    url_assets: String,
    url_exchanges: String,
    url_instruments: String,
    url_stats: String,
}

impl DownloadUtils {
    pub fn new(url: &str) -> Self {
        Self {
            client: util_client::get_client(),
            url_assets: DownloadUtils::format_url(url, "assets"),
            url_exchanges: DownloadUtils::format_url(url, "exchanges"),
            url_instruments: DownloadUtils::format_url(url, "instruments"),
            url_stats: DownloadUtils::format_url(url, "stats"),
        }
    }
}

impl DownloadUtils {
    /// Downloads a file from the specified URL and returns the content of the body as `Vec<u8>`.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice that holds the URL of the file to be downloaded.
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<u8>, DownloadError>` indicating the success or failure of the download operation.
    ///
    async fn download(&self, url: &str) -> Result<Vec<u8>, DownloadError> {
        let resp = self
            .client
            .get(url)
            .header("Accept", "application/json")
            .send()
            .await
            .expect("request failed");

        let body = resp.bytes().await.expect("body invalid");

        Ok(body.to_vec())
    }

    /// Formats the URL to be used in the request to the API
    ///
    /// # Arguments
    ///
    /// * `uri` - A string slice that holds the endpoint URI to be formatted
    ///
    fn format_url(url: &str, endpoint: &str) -> String {
        format!("{}{}", url, endpoint)
    }
}
