use db_utils::prelude::ClickHouseClient;
use db_utils::{ddl, query_utils};
use std::error::Error;

pub(crate) async fn setup_exchanges_table(client: &ClickHouseClient) -> Result<(), Box<dyn Error>> {
    let ddl = ddl::generate_create_exchanges_table_ddl();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create exchanges table");

    Ok(())
}
