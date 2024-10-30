use container_specs_postgres::postgres_db_container_config;
use docker_utils::prelude::DockerUtil;
use environment_manager::EnvironmentManager;
use kaiko_import::prelude::{execute_workflow, MetaDataDBWOp, WorkflowOp, WorkflowOpAll};
use kaiko_test_utils::utils_update;
use pg_mddb_manager::PostgresMDDBManager;
use postgres_config_manager::PostgresConfigManager;

// Somehow tests seem to be executed or sorted in alphabetical order, so make sure that the
// setup is on top of the stack.
#[tokio::test]
async fn all_setup() {
    let env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres container
    let container_config = postgres_db_container_config();
    let result = env.get_or_start_container_config(&container_config);
    // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_partial_update() {
    let config_manager = EnvironmentManager::new();
    let env_type = config_manager.env_type();

    let pg_cfg_manager = PostgresConfigManager::new(&env_type);
    let dsn = pg_cfg_manager.pg_connection_url();

    let dbm_mddb = PostgresMDDBManager::test_with_debug(&dsn, true)
        .await
        .expect("Failed to create PostgresSMDBManager");

    //
    // Pre update data import
    //
    let meta_data = utils_update::get_partial_pre_update_test_data_set();
    let workflow = get_full_import_op();

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

    //
    // Update data
    //
    // let asset_id = utils_update::get_partial_update_test_asset_id();
    // let exchange_id = utils_update::get_partial_update_test_exchange_id();
    // let instrument_id = utils_update::get_partial_update_test_instrument_id();
    //
    // // Update asset
    // let workflow = get_assets_update_op();
    // execute_workflow(&dbm_mddb, &meta_data, &workflow).await;
    //
    // // Load the updated asset from the DB
    // let res = dbm_mddb.read_asset(asset_id.clone()).await;
    // dbg!(&res);
    // assert!(res.is_ok());
    //
    // let db_asset = res.unwrap();
    // // Extract the reference asset from the test data set
    // let db_update_asset = meta_data.assets().data.first().unwrap();
    // // Compare the hashes as these must match after the update
    // assert_eq!(db_asset.hash(), db_update_asset.hash());
}

fn get_full_import_op() -> MetaDataDBWOp {
    let all_op: WorkflowOpAll = WorkflowOpAll::ImportAll;
    let assets_op: WorkflowOp = WorkflowOp::NoOP;
    let exchanges_op: WorkflowOp = WorkflowOp::NoOP;
    let instruments_op: WorkflowOp = WorkflowOp::NoOP;
    MetaDataDBWOp::new(all_op, assets_op, exchanges_op, instruments_op)
}

fn get_assets_update_op() -> MetaDataDBWOp {
    let all_op: WorkflowOpAll = WorkflowOpAll::UpdatePartial;
    let assets_op: WorkflowOp = WorkflowOp::UpdateAssets;
    let exchanges_op: WorkflowOp = WorkflowOp::NoOP;
    let instruments_op: WorkflowOp = WorkflowOp::NoOP;
    MetaDataDBWOp::new(all_op, assets_op, exchanges_op, instruments_op)
}

fn get_exchanges_update_op() -> MetaDataDBWOp {
    let all_op: WorkflowOpAll = WorkflowOpAll::UpdatePartial;
    let assets_op: WorkflowOp = WorkflowOp::NoOP;
    let exchanges_op: WorkflowOp = WorkflowOp::UpdateExchanges;
    let instruments_op: WorkflowOp = WorkflowOp::NoOP;
    MetaDataDBWOp::new(all_op, assets_op, exchanges_op, instruments_op)
}

fn get_instruments_update_op() -> MetaDataDBWOp {
    let all_op: WorkflowOpAll = WorkflowOpAll::UpdatePartial;
    let assets_op: WorkflowOp = WorkflowOp::NoOP;
    let exchanges_op: WorkflowOp = WorkflowOp::NoOP;
    let instruments_op: WorkflowOp = WorkflowOp::UpdateInstruments;
    MetaDataDBWOp::new(all_op, assets_op, exchanges_op, instruments_op)
}
