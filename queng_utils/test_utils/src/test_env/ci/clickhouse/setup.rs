use crate::prelude::{ContainerConfig, TestEnvError};
use crate::test_env::ci::clickhouse;
use crate::test_env::ci::metadata::{assets, exchanges, instruments};
use db_utils::prelude::ClickHouseClient;
use db_utils::{db, query_utils};

pub(crate) async fn configure_reset_or_reuse_clickhouse_db(
    client: &ClickHouseClient,
    container_config: &ContainerConfig<'_>,
) -> Result<(), TestEnvError> {
    // Check if DB is already configured
    let configured = is_clickhouse_configured(container_config);
    // Check if the container configuration should be re-set every time
    let reset_config = container_config.reset_configuration();

    // Container is *NOT* configured.
    if !configured {
        // Configure container from scratch.
        configure_clickhouse(client, false)
            .await
            .expect("Failed to configure ClickHouse");
    }

    // Configuration needs to be reset.
    if reset_config {
        // Reset all configuration to its initial state
        configure_clickhouse(client, true)
            .await
            .expect("Failed to reset and re-configure ClickHouse");
    }

    // Container is fully configured, no reset needed,
    // therefor its good to re-use. Just return OK.

    Ok(())
}

pub(crate) fn is_clickhouse_configured(_container_config: &ContainerConfig) -> bool {
    false
}

pub(crate) async fn configure_clickhouse(
    client: &ClickHouseClient,
    reset_config: bool,
) -> Result<(), TestEnvError> {
    // here we have to remove & re-create all databases to configure CH anew
    if reset_config {
        // Delete all databases
        clickhouse::shared::drop_system_db(client)
            .await
            .expect("Failed to drop SYSTEM DB in Clickhouse");
        clickhouse::shared::drop_metadata_db(client)
            .await
            .expect("Failed to drop METADATA DB in Clickhouse");
    }

    // Create al databases
    create_databases(client)
        .await
        .expect("Failed to create databases");

    // Create all tables
    create_tables(client)
        .await
        .expect("Failed to create tables");

    // Import all data
    import_data(client).await.expect("Failed to import data");

    Ok(())
}

async fn create_databases(client: &ClickHouseClient) -> Result<(), TestEnvError> {
    let ddl = db::create_system_db();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create SYSTEM DB in Clickhouse");

    let ddl = db::create_metadata_db();
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create METADATA DB in Clickhouse");

    Ok(())
}

async fn create_tables(client: &ClickHouseClient) -> Result<(), TestEnvError> {
    assets::setup_assets_table(client)
        .await
        .expect("Failed to create asset table");

    exchanges::setup_exchanges_table(client)
        .await
        .expect("Failed to create exchanges table");

    instruments::setup_instruments_table(client)
        .await
        .expect("Failed to create instruments table");

    Ok(())
}

async fn import_data(client: &ClickHouseClient) -> Result<(), TestEnvError> {
    assets::import_assets(client)
        .await
        .expect("Failed to import asset data");

    exchanges::import_exchanges(client)
        .await
        .expect("Failed to import exchanges data");

    instruments::import_instruments(client)
        .await
        .expect("Failed to import instruments data");

    Ok(())
}
