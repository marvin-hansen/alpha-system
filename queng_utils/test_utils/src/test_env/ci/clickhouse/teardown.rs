use crate::prelude::TestEnvError;
use crate::test_env::ci::clickhouse::shared;
use db_utils::prelude::ClickHouseClient;

pub(crate) async fn drop_databases(client: &ClickHouseClient) -> Result<(), TestEnvError> {
    shared::drop_system_db(client)
        .await
        .expect("Failed to drop SYSTEM DB in Clickhouse");

    shared::drop_metadata_db(client)
        .await
        .expect("Failed to drop METADATA DB in Clickhouse");

    Ok(())
}
