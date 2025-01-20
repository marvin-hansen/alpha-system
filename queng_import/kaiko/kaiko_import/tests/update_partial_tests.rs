use container_specs_postgres::postgres_db_container_config;
use docker_utils::DockerUtil;
use environment_manager::EnvironmentManager;
use kaiko_import::{execute_workflow, MetaDataDBWOp, WorkflowOp, WorkflowOpAll};
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
    let result = env.get_or_start_container(&container_config);
    // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_partial_update() {
    let config_manager = EnvironmentManager::new();
    let env_type = config_manager.env_type();

    let pg_cfg_manager = PostgresConfigManager::new(&env_type);
    let dsn = pg_cfg_manager.pg_connection_url();

    let dbm_mddb = PostgresMDDBManager::with_test_and_debug(&dsn)
        .await
        .expect("Failed to create PostgresSMDBManager");

    //
    // Import data
    //
    let meta_data = utils_update::get_pre_update_test_data_set();
    let workflow = get_full_import_op();

    execute_workflow(&dbm_mddb, &meta_data, &workflow).await;

    //
    // Verify data import
    //
    // Count if assets, exchanges, and instruments are in the DB.
    let result = dbm_mddb.count_assets().await;
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, 1);

    let result = dbm_mddb.count_exchanges().await;
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, 1);

    let result = dbm_mddb.count_instruments().await;
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, 1);

    // Load the inserted asset from the DB
    let asset_id = meta_data.assets().data.first().unwrap().code.clone();
    let res = dbm_mddb.read_asset(asset_id).await;
    // dbg!(&res);
    assert!(res.is_ok());
    let db_original_asset = res.unwrap().unwrap();

    //
    // Update metadata
    //
    let meta_data = utils_update::get_update_test_data_set();

    //
    // Update assets metadata
    //
    let asset_id = utils_update::get_partial_update_test_asset_id();
    let workflow = get_assets_update_op();
    execute_workflow(&dbm_mddb, &meta_data, &workflow).await;

    // Load the updated asset from the DB
    let res = dbm_mddb.read_asset(asset_id.clone()).await;
    // dbg!(&res);
    assert!(res.is_ok());

    // Verify the asset has been updated
    let db_updated_asset = res.unwrap().unwrap();
    // Extract the reference asset from the test data set
    let db_update_asset = meta_data.assets().data.first().unwrap();
    // Compare the hash of the reference asset in the test data set to the updated asset, which must be equal.
    assert_eq!(db_updated_asset.hash(), db_update_asset.hash());
    // Compare the hash of the original asset in the database to the updated asset, which must be different (ne= not equal)
    assert_ne!(db_original_asset.hash(), db_update_asset.hash());

    // Load the exchange metadata from the DB
    let exchange_id = meta_data.exchanges().data.first().unwrap().code.clone();
    let res = dbm_mddb.read_exchange(exchange_id).await;
    // dbg!(&res);
    assert!(res.is_ok());
    let db_original_exchange = res.unwrap().unwrap();

    //
    // Update exchanges metadata
    //
    let exchange_id = utils_update::get_partial_update_test_exchange_id();
    let workflow = get_exchanges_update_op();
    execute_workflow(&dbm_mddb, &meta_data, &workflow).await;

    // Load the updated exchange from the DB
    let res = dbm_mddb.read_exchange(exchange_id.clone()).await;
    // dbg!(&res);
    assert!(res.is_ok());
    let db_updated_exchange = res.unwrap().unwrap();

    // Extract the reference exchange from the test data set
    let db_update_exchange = meta_data.exchanges().data.first().unwrap();
    // Compare the hash of the reference exchange in the test data set to the updated exchange, which must be equal.
    assert_eq!(db_updated_exchange.hash(), db_update_exchange.hash());
    // Compare the hash of the original exchange in the database to the updated exchange, which must be different (ne= not equal)
    assert_ne!(db_original_exchange.hash(), db_update_exchange.hash());

    // Load the instrument metadata from the DB
    let instrument_id = meta_data.instruments().data.first().unwrap().primary_key();
    let res = dbm_mddb.read_instrument(&instrument_id).await;
    dbg!(&res);
    assert!(res.is_ok());
    let db_original_instrument = res.unwrap().unwrap();

    //
    // Update instruments metadata
    //
    let instrument_id = utils_update::get_partial_update_test_instrument_id();
    let workflow = get_instruments_update_op();
    execute_workflow(&dbm_mddb, &meta_data, &workflow).await;

    // Load the updated instrument from the DB
    let res = dbm_mddb.read_instrument(&instrument_id.clone()).await;
    dbg!(&res);
    assert!(res.is_ok());
    let db_updated_instrument = res.unwrap().unwrap();

    // Extract the reference instrument from the test data set
    let update_instrument = meta_data.instruments().data.first().unwrap();
    // Compare the hash of the reference instrument in the test data set to the updated instrument, which must be equal.
    assert_eq!(db_updated_instrument.hash(), update_instrument.hash());
    // Compare the hash of the original instrument in the database to the updated instrument, which must be different (ne= not equal)
    assert_ne!(db_original_instrument.hash(), update_instrument.hash());
}

const fn get_full_import_op() -> MetaDataDBWOp {
    let all_op: WorkflowOpAll = WorkflowOpAll::ImportAll;
    let assets_op: WorkflowOp = WorkflowOp::NoOP;
    let exchanges_op: WorkflowOp = WorkflowOp::NoOP;
    let instruments_op: WorkflowOp = WorkflowOp::NoOP;
    MetaDataDBWOp::new(all_op, assets_op, exchanges_op, instruments_op)
}

const fn get_assets_update_op() -> MetaDataDBWOp {
    let all_op: WorkflowOpAll = WorkflowOpAll::UpdatePartial;
    let assets_op: WorkflowOp = WorkflowOp::UpdateAssets;
    let exchanges_op: WorkflowOp = WorkflowOp::NoOP;
    let instruments_op: WorkflowOp = WorkflowOp::NoOP;
    MetaDataDBWOp::new(all_op, assets_op, exchanges_op, instruments_op)
}

const fn get_exchanges_update_op() -> MetaDataDBWOp {
    let all_op: WorkflowOpAll = WorkflowOpAll::UpdatePartial;
    let assets_op: WorkflowOp = WorkflowOp::NoOP;
    let exchanges_op: WorkflowOp = WorkflowOp::UpdateExchanges;
    let instruments_op: WorkflowOp = WorkflowOp::NoOP;
    MetaDataDBWOp::new(all_op, assets_op, exchanges_op, instruments_op)
}

const fn get_instruments_update_op() -> MetaDataDBWOp {
    let all_op: WorkflowOpAll = WorkflowOpAll::UpdatePartial;
    let assets_op: WorkflowOp = WorkflowOp::NoOP;
    let exchanges_op: WorkflowOp = WorkflowOp::NoOP;
    let instruments_op: WorkflowOp = WorkflowOp::UpdateInstruments;
    MetaDataDBWOp::new(all_op, assets_op, exchanges_op, instruments_op)
}
