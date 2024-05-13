use crate::error::DownloadError;
use std::process::Command;

mod error;

pub const ASSETS_URL: &str = "'https://reference-data-api.kaiko.io/v1/assets'";
pub const ASSETS_FILE: &str = "assets.json";

fn main() {
    println!("Hello, world!");
}

async fn download_assets() -> Result<(), DownloadError> {
    // curl --compressed -H 'Accept: application/json' 'https://reference-data-api.kaiko.io/v1/assets' > assets.json
    return match Command::new("curl")
        .arg("--compressed")
        .arg("-H")
        .arg("'Accept: application/json'")
        .arg(ASSETS_URL)
        .arg(">")
        .arg(ASSETS_FILE)
        .status()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(DownloadError::from(format!(
            "Error downloading assets {}",
            e.to_string()
        ))),
    };
}
