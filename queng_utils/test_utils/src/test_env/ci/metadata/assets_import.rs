use db_utils::prelude::ClickHouseClient;
use db_utils::{insert, query_utils, types::Asset};
use std::error::Error;

pub(crate) async fn import_assets(
    client: &ClickHouseClient,
    assets: &Vec<Asset>,
) -> Result<(), Box<dyn Error>> {
    for asset in assets.iter() {
        let insert_query = insert::generate_asset_insert(asset);
        query_utils::execute_query(client, &insert_query)
            .await
            .expect("Failed to insert asset")
    }

    Ok(())
}
