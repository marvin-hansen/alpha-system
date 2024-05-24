use crate::errors::DownloadError;
use crate::fields::{ASSETS_DOWNLOAD_FILE, EXCHANGES_DOWNLOAD_FILE, INSTRUMENTS_DOWNLOAD_FILE};
use common::prelude::{Asset, Exchange, Instrument};
use std::fs::File;
use std::io::Write;
use std::path::Path;

// Function to save exchanges to a JSON file
pub(crate) async fn save_to_json(
    exchanges: &Vec<String>,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert the data vector to JSON
    let json =
        serde_json::to_string_pretty(exchanges).expect("Error converting data vector to JSON");

    // Create a file
    let mut file = File::create(filename).expect("Error creating JSON file");

    // Write the JSON to a file
    file.write_all(json.as_bytes())
        .expect("Error writing JSON to file");

    Ok(())
}

pub(crate) async fn load_assets() -> Result<Vec<Asset>, DownloadError> {
    let json_file_path = Path::new(ASSETS_DOWNLOAD_FILE);
    let file = File::open(json_file_path).expect("Error while reading assets date file");

    let assets: Vec<Asset> =
        serde_json::from_reader(file).expect("Error while parsing assets date file");

    Ok(assets)
}

pub(crate) async fn load_exchanges() -> Result<Vec<Exchange>, DownloadError> {
    let json_file_path = Path::new(EXCHANGES_DOWNLOAD_FILE);
    let file = File::open(json_file_path).expect("Error while reading exchange date file");

    let exchanges: Vec<Exchange> =
        serde_json::from_reader(file).expect("Error while parsing exchange date file");

    Ok(exchanges)
}

pub(crate) async fn load_instruments() -> Result<Vec<Instrument>, DownloadError> {
    let json_file_path = Path::new(INSTRUMENTS_DOWNLOAD_FILE);
    let file = File::open(json_file_path).expect("Error while reading instruments date file");

    let instruments: Vec<Instrument> =
        serde_json::from_reader(file).expect("Error while parsing instruments date file");

    Ok(instruments)
}
