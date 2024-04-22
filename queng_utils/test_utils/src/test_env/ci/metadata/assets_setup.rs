use db_utils::prelude::{ddl, query_utils, ClickHouseClient};
use std::error::Error;

pub(crate) async fn setup_assets_table(client: &ClickHouseClient) -> Result<(), Box<dyn Error>> {
    let ddl = ddl::generate_create_asset_table_ddl();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create asset table");

    Ok(())
}
