use db_utils::prelude::ClickHouseClient;
use db_utils::{ddl, query_utils};
use std::error::Error;

pub(crate) async fn teardown_instruments_table(
    client: &ClickHouseClient,
) -> Result<(), Box<dyn Error>> {
    let ddl = ddl::generate_drop_instruments_table_ddl();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to drop asset table");

    Ok(())
}
