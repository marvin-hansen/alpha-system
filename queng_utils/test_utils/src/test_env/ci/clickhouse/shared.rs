use db_utils::prelude::ClickHouseClient;

use db_utils::{db, query_utils};
use std::error::Error;

pub(crate) async fn drop_system_db(client: &ClickHouseClient) -> Result<(), Box<dyn Error>> {
    let ddl = db::drop_system_db();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create asset table");

    Ok(())
}

pub(crate) async fn drop_metadata_db(client: &ClickHouseClient) -> Result<(), Box<dyn Error>> {
    let ddl = db::drop_metadata_db();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create asset table");

    Ok(())
}
