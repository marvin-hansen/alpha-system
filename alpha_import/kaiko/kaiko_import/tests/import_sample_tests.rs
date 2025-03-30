/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use container_specs_postgres::postgres_db_container_config;
use docker_utils::DockerUtil;
use environment_manager::EnvironmentManager;
use kaiko_import::{MetaDataDBWOp, WorkflowOp, WorkflowOpAll, execute_workflow};
use kaiko_test_utils::utils_import;
use pg_mddb_manager::PostgresMDDBManager;
use postgres_config_manager::PostgresConfigManager;

// Somehow tests seem to be executed or sorted in alphabetical order, so make sure that the
// setup is on top of the stack.
#[tokio::test]
async fn all_setup() {
    let env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres container
    let container_config = postgres_db_container_config();
    let result = env.get_or_start_container(&container_config);
    // dbg!(&result);
    assert!(result.is_ok());
}
#[tokio::test]
async fn test_sample_import() {
    let config_manager = EnvironmentManager::new();
    let env_type = config_manager.env_type();

    let pg_cfg_manager = PostgresConfigManager::new(&env_type);
    let dsn = pg_cfg_manager.pg_connection_url();

    let dbm_mddb = PostgresMDDBManager::with_test_and_debug(&dsn)
        .await
        .expect("Failed to create PostgresSMDBManager");

    let meta_data = utils_import::get_full_import_test_data_set();
    let workflow = get_import_sample_op();

    execute_workflow(&dbm_mddb, &meta_data, &workflow).await;

    let result = dbm_mddb.count_assets().await;
    dbg!(&result);
    assert!(result.is_ok());

    let count = result.unwrap();
    assert_eq!(count, 1);

    let result = dbm_mddb.count_exchanges().await;
    dbg!(&result);
    assert!(result.is_ok());

    let count = result.unwrap();
    assert_eq!(count, 1);

    let result = dbm_mddb.count_instruments().await;
    dbg!(&result);
    assert!(result.is_ok());

    let count = result.unwrap();
    assert_eq!(count, 1);
}

const fn get_import_sample_op() -> MetaDataDBWOp {
    let assets_sample_size = 1;
    let exchanges_sample_size = 1;
    let instruments_sample_size = 1;

    let all_op: WorkflowOpAll = WorkflowOpAll::ImportSample(
        assets_sample_size,
        exchanges_sample_size,
        instruments_sample_size,
    );
    let assets_op: WorkflowOp = WorkflowOp::NoOP;
    let exchanges_op: WorkflowOp = WorkflowOp::NoOP;
    let instruments_op: WorkflowOp = WorkflowOp::NoOP;
    MetaDataDBWOp::new(all_op, assets_op, exchanges_op, instruments_op)
}
