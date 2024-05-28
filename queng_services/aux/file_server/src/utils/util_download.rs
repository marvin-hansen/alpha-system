use crate::errors::DownloadError;
use crate::fields::{
    ASSETS_DOWNLOAD_FILE, BASE_URL, EXCHANGES_DOWNLOAD_FILE, INSTRUMENTS_DOWNLOAD_FILE,
};
use std::process::Command;

pub(crate) async fn download_assets() -> Result<(), DownloadError> {
    // curl --compressed -H 'Accept: application/json' 'https://reference-data-api.kaiko.io/v1/assets' > assets.json
    let url = format!("'{}assets' ", BASE_URL);
    let out_file = ASSETS_DOWNLOAD_FILE;
    return match download(&url, out_file).await {
        Ok(_) => Ok(()),
        Err(e) => Err(DownloadError::from(format!(
            "Error downloading assets {}",
            e.to_string()
        ))),
    };
}

pub(crate) async fn download_exchanges() -> Result<(), DownloadError> {
    // curl --compressed -H 'Accept: application/json' 'https://reference-data-api.kaiko.io/v1/assets' > assets.json
    let url = format!("'{}exchanges' ", BASE_URL);
    let out_file = EXCHANGES_DOWNLOAD_FILE;
    return match download(&url, out_file).await {
        Ok(_) => Ok(()),
        Err(e) => Err(DownloadError::from(format!(
            "Error downloading exchanges {}",
            e.to_string()
        ))),
    };
}

pub(crate) async fn download_instruments() -> Result<(), DownloadError> {
    // curl --compressed -H 'Accept: application/json' 'https://reference-data-api.kaiko.io/v1/assets' > assets.json
    let url = format!("'{}instruments' ", BASE_URL);
    let out_file = INSTRUMENTS_DOWNLOAD_FILE;
    return match download(&url, out_file).await {
        Ok(_) => Ok(()),
        Err(e) => Err(DownloadError::from(format!(
            "Error downloading instruments {}",
            e.to_string()
        ))),
    };
}

/// Downloads a file from the specified URL using the curl command and saves it to the given output path.
///
/// This asynchronous function uses the `curl` command-line tool to download the content
/// from the provided `url` and saves it to `out_file`. If the download is successful,
/// the function returns `Ok(())`. If an error occurs during the download process,
/// a `DownloadError` is returned.
///
/// # Arguments
///
/// * `url` - A string slice that holds the URL of the file to be downloaded.
/// * `out_file` - A string slice that specifies the path where the downloaded file should be saved.
///
/// # Returns
///
/// Returns `Result<(), DownloadError>` indicating the success or failure of the download operation.
///
async fn download(url: &str, out_file: &str) -> Result<(), DownloadError> {
    println!("url: {}", url);
    println!("out_file: {}", out_file);

    return match Command::new("sh")
        .arg("/usr/bin/curl")
        .arg("--compressed")
        .arg("-H")
        .arg("'Accept: application/json'")
        .arg(url)
        .arg(">")
        .arg(out_file)
        .status()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(DownloadError::from(e.to_string())),
    };
}
