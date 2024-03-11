use client_utils::{config_utils, file_utils, print_utils};
use common::prelude::ClickHouseConfig;
use klickhouse::{Client, ClientOptions};
use std::time::Instant;

const VERBOSE: bool = true;

const CONFIG_FILE_NAME: &str = "import_metadata_config.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vrb = VERBOSE;

    let start = Instant::now();
    print_utils::print_symbol_meta_data_import_header();

    print_utils::dbg_print(vrb, "Build DB Client");
    let db_config = ClickHouseConfig::default();
    let destination = db_config.connection_string();
    let client = Client::connect(destination.clone(), ClientOptions::default())
        .await
        .expect(format!("Failed to connect to {}", &destination).as_str());

    print_utils::dbg_print(vrb, "Build import config");
    let config =
        config_utils::get_config_file(CONFIG_FILE_NAME).expect("Import config file not found");

    print_utils::dbg_print(
        vrb,
        format!("Import data folder: {}", config.data_folder()).as_str(),
    );

    print_utils::dbg_print(vrb, "Read all files in data folder");
    let files = file_utils::get_file_paths_from_directory(config.data_folder())
        .expect("Failed to read files in data folder");

    print_utils::dbg_print(vrb, format!("Found {} files", files.len()).as_str());

    for file in files {
        println!("Importing file: {}", file.to_str().unwrap());
    }

    print_utils::print_duration(&start.elapsed());
    Ok(())
}
