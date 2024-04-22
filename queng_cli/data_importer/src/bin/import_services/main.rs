mod process;

use client_utils::print_utils;
use common::prelude::ClickHouseConfig;
use klickhouse::{Client, ClientOptions};
use std::time::Instant;

const VERBOSE: bool = true;
// const TABLE_NAME: &str = "system.services";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    print_utils::print_data_import_header("Imports service meta data into Clickhouse.");
    let vrb = VERBOSE;

    print_utils::dbg_print(vrb, "Build DB Client");
    let db_config = ClickHouseConfig::default();
    let destination = db_config.connection_string();
    let client = Client::connect(destination.clone(), ClientOptions::default())
        .await
        .expect(format!("Failed to connect to {}", &destination).as_str());

    print_utils::dbg_print(vrb, "Process data import");
    process::process(&client, vrb)
        .await
        .expect("Failed to import data");

    print_utils::print_duration(&start.elapsed());
    Ok(())
}
