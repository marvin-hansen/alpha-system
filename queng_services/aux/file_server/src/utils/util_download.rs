use crate::errors::DownloadError;
use crate::fields::BASE_URL;
use common::prelude::{Asset, AssetRoot, Exchange, ExchangesRoot, Instrument, InstrumentsRoot};

pub(crate) async fn download_assets() -> Result<Vec<Asset>, DownloadError> {
    // curl --compressed -H 'Accept: application/json' 'https://reference-data-api.kaiko.io/v1/assets' > assets.json
    let url = format!("{}assets", BASE_URL);
    return match download(&url).await {
        Ok(bytes) => {
            let assets: AssetRoot =
                serde_json::from_slice(bytes.as_slice()).expect("Failed to parse assets");

            Ok(assets.data)
        }
        Err(e) => Err(DownloadError::from(format!(
            "Error downloading assets {}",
            e.to_string()
        ))),
    };
}

pub(crate) async fn download_exchanges() -> Result<Vec<Exchange>, DownloadError> {
    // curl --compressed -H 'Accept: application/json' 'https://reference-data-api.kaiko.io/v1/assets' > assets.json
    let url = format!("{}exchanges", BASE_URL);
    return match download(&url).await {
        Ok(bytes) => {
            let exchanges: ExchangesRoot =
                serde_json::from_slice(bytes.as_slice()).expect("Failed to parse exchanges");

            Ok(exchanges.data)
        }
        Err(e) => Err(DownloadError::from(format!(
            "Error downloading exchanges {}",
            e.to_string()
        ))),
    };
}

pub(crate) async fn download_instruments() -> Result<Vec<Instrument>, DownloadError> {
    // curl --compressed -H 'Accept: application/json' 'https://reference-data-api.kaiko.io/v1/assets' > assets.json
    let url = format!("{}instruments", BASE_URL);
    return match download(&url).await {
        Ok(bytes) => {
            let instruments: InstrumentsRoot =
                serde_json::from_slice(bytes.as_slice()).expect("Failed to parse exchanges");

            Ok(instruments.data)
        }
        Err(e) => Err(DownloadError::from(format!(
            "Error downloading instruments {}",
            e.to_string()
        ))),
    };
}

/// Downloads a file from the specified URL and returns the content of the body as Vec<u8> .
///
///
/// # Arguments
///
/// * `url` - A string slice that holds the URL of the file to be downloaded.
///
/// # Returns
///
/// Returns `Result<Vec<u8>, DownloadError>` indicating the success or failure of the download operation.
///
async fn download(url: &str) -> Result<Vec<u8>, DownloadError> {
    // Enable gzip compressions for requests and responses to reduce download time.
    // https://dtantsur.github.io/rust-openstack/reqwest/struct.ClientBuilder.html
    let client = reqwest::Client::builder()
        .gzip(true)
        .build()
        .expect("Failed to build reqwest client");

    let resp = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await
        .expect("request failed");

    let body = resp.bytes().await.expect("body invalid");

    Ok(body.to_vec())
}
