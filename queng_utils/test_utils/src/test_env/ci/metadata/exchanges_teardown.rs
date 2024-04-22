use db_utils::prelude::ClickHouseClient;
use db_utils::{ddl, query_utils};
use std::error::Error;

pub(crate) async fn teardown_exchanges_table(
    client: &ClickHouseClient,
) -> Result<(), Box<dyn Error>> {
    let ddl = ddl::generate_drop_exchanges_table_ddl();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to drop exchanges table");

    Ok(())
}
