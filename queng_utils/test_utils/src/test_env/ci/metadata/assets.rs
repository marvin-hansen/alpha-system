use db_utils::prelude::ClickHouseClient;
use db_utils::types::AssetRoot;
use db_utils::{ddl, insert, query_utils, types::Asset};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub(crate) async fn setup_assets_table(client: &ClickHouseClient) -> Result<(), Box<dyn Error>> {
    let ddl = ddl::generate_create_asset_table_ddl();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create asset table");

    Ok(())
}

pub(crate) async fn import_assets(client: &ClickHouseClient) -> Result<(), Box<dyn Error>> {
    let path = "";
    let assets = load_assets(path)
        .await
        .expect("Failed to load assets.json file");

    for asset in assets.iter() {
        let insert_query = insert::generate_asset_insert(asset);
        query_utils::execute_query(client, &insert_query)
            .await
            .expect("Failed to insert asset")
    }

    Ok(())
}

async fn load_assets(path: &str) -> Result<Vec<Asset>, Box<dyn Error>> {
    let file_path = PathBuf::from(path);
    let file = File::open(file_path).expect("file not found");
    let assets: AssetRoot = serde_json::from_reader(file).expect("error while reading");
    Ok(assets.data)
}
