use crate::fields::VALID_EXCHANGES_DOWNLOAD_FILE;
use crate::utils::{util_json, util_scraping};
use std::error::Error;

mod fields;
mod init;
mod service;
mod types;
mod utils;

const VRB: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Scraping valid exchanges");
    let valid_exchanges = util_scraping::scrap_valid_exchanges(VRB)
        .await
        .expect("Error scraping valid exchanges");

    println!("Saving valid exchanges to JSON file");
    util_json::save_to_json(&valid_exchanges, VALID_EXCHANGES_DOWNLOAD_FILE)
        .await
        .expect("Error saving valid exchanges to JSON file");

    println!(
        "Saved {} exchanges to {}",
        valid_exchanges.len(),
        VALID_EXCHANGES_DOWNLOAD_FILE
    );

    println!("Done");
    Ok(())
}
