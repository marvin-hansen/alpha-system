mod config;
mod process;
mod process_assets;
mod process_exchanges;
mod process_instruments;

use client_utils::{file_utils, print_utils};
use common::prelude::ClickHouseConfig;
use klickhouse::{Client, ClientOptions};
use std::time::Instant;

const VERBOSE: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vrb = VERBOSE;

    let start = Instant::now();
    print_utils::print_data_import_header("Imports symbol meta data into Clickhouse.");

    print_utils::dbg_print(vrb, "Build DB Client");
    let db_config = ClickHouseConfig::default();
    let destination = db_config.connection_string();

    print_utils::dbg_print(vrb, "Connect to DB");
    let client = Client::connect(destination.clone(), ClientOptions::default())
        .await
        .expect(format!("Failed to connect to {}", &destination).as_str());

    print_utils::dbg_print(vrb, "Build import config");
    let config = config::get_meta_data_config();

    print_utils::dbg_print(
        vrb,
        format!("Import data folder: {}", config.data_folder()).as_str(),
    );

    print_utils::dbg_print(vrb, "Read all files in data folder");
    let files = file_utils::get_file_paths_from_directory(config.data_folder())
        .expect("Failed to read files in data folder");

    print_utils::dbg_print(vrb, format!("Found {} files", files.len()).as_str());

    for file in files {
        if file.to_str().unwrap().contains(".json") {
            process::process(&client, &file, vrb)
                .await
                .expect("Failed to import file");
        }
    }

    print_utils::print_duration(&start.elapsed());
    Ok(())
}
