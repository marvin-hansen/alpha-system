use crate::env_util::ci::clickhouse::shared;
use crate::prelude::EnvironmentError;
use db_utils::prelude::ClickHouseClient;

pub(crate) async fn drop_databases(client: &ClickHouseClient) -> Result<(), EnvironmentError> {
    shared::drop_system_db(client)
        .await
        .expect("Failed to drop SYSTEM DB in Clickhouse");

    shared::drop_metadata_db(client)
        .await
        .expect("Failed to drop METADATA DB in Clickhouse");

    Ok(())
}
