use client_utils::print_utils;
use db_utils::{
    ddl, insert, query_utils,
    types::{Asset, AssetRoot},
};
use klickhouse::Client;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub(crate) async fn process_assets(
    client: &Client,
    file_path: &PathBuf,
    vrb: bool,
) -> Result<(), Box<dyn Error>> {
    print_utils::dbg_print(vrb, "Processing assets");

    print_utils::dbg_print(vrb, "Load assets from files");
    let assets = get_assets_from_file(file_path)
        .await
        .expect("Failed to read assets from file");

    let ddl = ddl::generate_create_asset_table_ddl();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create asset table");

    print_utils::dbg_print(vrb, "Importing assets");
    for asset in assets.iter() {
        let insert_query = insert::generate_asset_insert(asset);
        query_utils::execute_query(client, &insert_query)
            .await
            .expect("Failed to insert asset")
    }

    let count = assets.len();
    println!("Number of assets: {}", count);

    let count = query_utils::count_rows(client, "default.assets")
        .await
        .expect("Failed to count rows");

    println!("Number of assets imported: {}", count);

    Ok(())
}

async fn get_assets_from_file(file_path: &PathBuf) -> Result<Vec<Asset>, Box<dyn Error>> {
    let file = File::open(file_path).expect("file not found");
    let assets: AssetRoot = serde_json::from_reader(file).expect("error while reading");
    Ok(assets.data)
}
