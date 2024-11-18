use common_config::ServiceID;
use common_env::EnvironmentType;
use container_specs_postgres::postgres_db_container_config;
use docker_utils::prelude::DockerUtil;
use mddb_client::MDDBClient;
use metadata_import::MetadataImportManager;
use service_import::ServiceImportManager;
use service_utils::{ServiceUtil, ServiceWaitStrategy};
use std::time::Duration;

const ASSETS_SAMPLE_SIZE: usize = 50;
const EXCHANGES_SAMPLE_SIZE: usize = 50;
//  We need to import more instruments b/c the first 50 do not have FIGI ID's assigned.
const INSTRUMENTS_SAMPLE_SIZE: usize = 500;

#[tokio::test]
async fn test_mddb() {
    let docker_util = DockerUtil::with_debug().expect("Failed to get DockerUtil");
    // Start service util
    let res = ServiceUtil::with_debug().await;
    dbg!(&res);
    assert!(res.is_ok());
    let svc_util = res.unwrap();

    // Get config manger for automatic configuration
    let config_manager = svc_util.config_manager();

    let env_type = config_manager.env_type();

    // Start or reuse a test postgres database container
    let pg_container_config = postgres_db_container_config();
    let result = docker_util.get_or_start_container_config(&pg_container_config);
    dbg!(&result);
    assert!(result.is_ok());
    let (pg_container_id, _) = result.unwrap();

    // Test if service data is already imported in the DB; if not, do so.
    let service_import_manager = ServiceImportManager::with_debug().await;
    let imported = service_import_manager.check_if_already_imported().await;

    if !imported {
        service_import_manager
            .import_services()
            .await
            .expect("Failed to import services");
    }

    let imported = service_import_manager.check_if_already_imported().await;
    assert!(imported);

    //Determine workflow for metadata import
    let meta_data_import_manager = MetadataImportManager::with_debug().await;

    // Import a sample of 50 metadata records for each type
    let workflow = meta_data_import_manager
        .determine_workflow(Some((
            ASSETS_SAMPLE_SIZE,
            EXCHANGES_SAMPLE_SIZE,
            INSTRUMENTS_SAMPLE_SIZE,
        )))
        .await
        .expect("Failed to determine workflow");

    dbg!(&workflow);

    // Execute workflow
    meta_data_import_manager
        .execute_workflow(&workflow)
        .await
        .expect("Failed to execute workflow");

    // Wait for services to be ready
    let wait_strategy = if env_type == EnvironmentType::LOCAL {
        ServiceWaitStrategy::Duration(Duration::from_millis(250))
    } else {
        ServiceWaitStrategy::Duration(Duration::from_millis(500))
    };

    // Start DBGW service - depends on Database
    let service_id = ServiceID::DBGW;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    dbg!(&result);
    assert!(result.is_ok());

    // Start SMDB service - depends on DBGW
    let service_id = ServiceID::SMDB;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    assert!(result.is_ok());

    // Start MDDB service - depends on SMDB
    let service_id = ServiceID::MDDB;
    // Somehow MDDB needs more time than others to start
    let wait_strategy = ServiceWaitStrategy::Duration(Duration::from_millis(500));
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    assert!(result.is_ok());

    // Configure MDDB client
    let (host, port) = config_manager
        .get_mddb_host_port()
        .await
        .expect("Failed to get MDDB host");
    dbg!(&host);
    dbg!(&port);

    // Construct MDDB client
    let client = MDDBClient::new(host, port)
        .await
        .expect("Failed to create MDDB client");

    // Test MDDB Assets methods.
    test_metadata_assets_api(&client).await;

    // Test MDDB Exchanges methods.
    test_metadata_exchanges_api(&client).await;

    // Test MDDB Instruments methods.
    test_metadata_instruments_api(&client).await;

    // Stop and remove container
    let result = docker_util.stop_container(&pg_container_id);
    dbg!(&result);
    assert!(result.is_ok());
}

async fn test_metadata_assets_api(mddb_client: &MDDBClient) {
    // Test count_assets
    let result = mddb_client.count_assets().await;
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, ASSETS_SAMPLE_SIZE as u64);

    // Test check_if_asset_id_exists - success case i,e, exists
    let exists_id = "42";
    let result = mddb_client.check_if_asset_id_exists(exists_id).await;
    assert!(result.is_ok());
    let exists = result.unwrap();
    assert!(exists);

    // Test check_if_asset_id_exists - Fail case i,e, does not exists.
    let does_not_exists_id = "zztopxyz_non_exist";
    let result = mddb_client
        .check_if_asset_id_exists(does_not_exists_id)
        .await;
    assert!(result.is_ok());
    let exists = result.unwrap();
    assert!(!exists);

    // Test get_asset - success case
    let result = mddb_client.get_asset("42").await;
    assert!(result.is_ok());
    let asset = result.unwrap();
    assert!(asset.is_some());

    // Test get_asset - fail case
    let result = mddb_client.get_asset("zztopxyz_non_exist").await;
    assert!(result.is_ok());
    let asset = result.unwrap();
    assert!(asset.is_none());

    // Test get_all_assets - success case
    let result = mddb_client.get_all_assets().await;
    assert!(result.is_ok());
    let assets = result.unwrap();
    assert!(!assets.is_empty());
    let len = assets.len();
    assert_eq!(len, 50);
}

async fn test_metadata_exchanges_api(mddb_client: &MDDBClient) {
    // Test count_exchanges
    let result = mddb_client.count_exchanges().await;
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, EXCHANGES_SAMPLE_SIZE as u64);

    // Test check_if_exchange_id_exists - success case i,e, exists
    let exists_id = "bbit";
    let result = mddb_client.check_if_exchange_id_exists(exists_id).await;
    assert!(result.is_ok());
    let exists = result.unwrap();
    assert!(exists);

    // Test check_if_exchange_id_exists - Fail case i,e, does not exists.
    let does_not_exists_id = "zztopxyz_non_exist";
    let result = mddb_client
        .check_if_exchange_id_exists(does_not_exists_id)
        .await;
    assert!(result.is_ok());
    let exists = result.unwrap();
    assert!(!exists);

    // Test get_exchange - success case
    let result = mddb_client.get_exchange("bbit").await;
    assert!(result.is_ok());
    // let exchange = result.unwrap();
    // assert!(exchange.is_some());

    // Test get_exchange - fail case
    let result = mddb_client.get_exchange("zztopxyz_non_exist").await;
    assert!(result.is_err());

    // Test get_all_exchanges - success case
    let result = mddb_client.get_all_exchanges().await;
    assert!(result.is_ok());
    let exchanges = result.unwrap();
    assert!(!exchanges.is_empty());
    let len = exchanges.len();
    assert_eq!(len, 50);
}

async fn test_metadata_instruments_api(mddb_client: &MDDBClient) {
    // Test count_instruments
    let result = mddb_client.count_instruments().await;
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count, INSTRUMENTS_SAMPLE_SIZE as u64);

    // Test check_if_instrument_id_exists - success case i,e, exists
    let exists_id = "bbit_perpetual-future_btc_usd";
    let result = mddb_client.check_if_instrument_id_exists(exists_id).await;
    assert!(result.is_ok());
    let exists = result.unwrap();
    assert!(exists);

    // Test check_if_instrument_id_exists - Fail case i,e, does not exists.
    let does_not_exists_id = "zztopxyz_non_exist";
    let result = mddb_client
        .check_if_instrument_id_exists(does_not_exists_id)
        .await;
    assert!(result.is_ok());
    let exists = result.unwrap();
    assert!(!exists);

    // Test get_instrument - success case
    let exists_id = "bbit_perpetual-future_btc_usd";
    let result = mddb_client.get_instrument(exists_id).await;
    assert!(result.is_ok());
    let instrument = result.unwrap();
    assert!(instrument.is_some());

    // Test get_instrument - fail case
    let does_not_exists_id = "zztopxyz_non_exist";
    let result = mddb_client.get_instrument(does_not_exists_id).await;
    assert!(result.is_ok());
    let instrument = result.unwrap();
    assert!(instrument.is_none());

    // Test get_instrument_by_figi - success case
    let exists_figi = "KKG00000V307";
    let result = mddb_client.get_instrument_by_figi(exists_figi).await;
    assert!(result.is_ok());
    let instrument = result.unwrap();
    assert!(instrument.is_some());

    // Test get_instrument_by_figi - fail case
    let does_not_exists_figi = "zztopxyz_non_exist";
    let result = mddb_client
        .get_instrument_by_figi(does_not_exists_figi)
        .await;
    assert!(result.is_ok());
    let instrument = result.unwrap();
    assert!(instrument.is_none());

    // Test get_instrument_by_pair_figi - success case
    let exists_figi = "KKG000000SC3";
    let result = mddb_client.get_instrument_by_pair_figi(exists_figi).await;
    assert!(result.is_ok());
    let instrument = result.unwrap();
    assert!(instrument.is_some());

    // Test get_instrument_by_pair_figi - fail case
    let does_not_exists_figi = "zztopxyz_non_exist";
    let result = mddb_client
        .get_instrument_by_pair_figi(does_not_exists_figi)
        .await;
    assert!(result.is_ok());
    let instrument = result.unwrap();
    assert!(instrument.is_none());

    // Test get_all_instruments - success case
    let result = mddb_client.get_all_instruments().await;
    assert!(result.is_ok());
    let instruments = result.unwrap();
    assert!(!instruments.is_empty());
    let len = instruments.len();
    assert_eq!(len, INSTRUMENTS_SAMPLE_SIZE);

    // Test get_all_instruments_for_base_asset - success case
    let exists_base_asset = "btc";
    let result = mddb_client
        .get_all_instruments_for_base_asset(exists_base_asset)
        .await;
    assert!(result.is_ok());
    let instruments = result.unwrap();
    assert!(!instruments.is_empty());
    let len = instruments.len();
    assert!(len > 1);

    // Test get_all_instruments_for_base_asset - fail case
    let does_not_exists_base_asset = "zztopxyz_non_exist";
    let result = mddb_client
        .get_all_instruments_for_base_asset(does_not_exists_base_asset)
        .await;
    assert!(result.is_ok());
    let instruments = result.unwrap();
    assert!(instruments.is_empty());
    let len = instruments.len();
    assert_eq!(len, 0);

    // Test get_all_instruments_for_quote_asset - success case
    let exists_quote_asset = "usdt";
    let result = mddb_client
        .get_all_instruments_for_quote_asset(exists_quote_asset)
        .await;
    assert!(result.is_ok());
    let instruments = result.unwrap();
    assert!(!instruments.is_empty());
    let len = instruments.len();
    assert!(len > 1);

    // Test get_all_instruments_for_quote_asset - fail case
    let does_not_exists_quote_asset = "zztopxyz_non_exist";
    let result = mddb_client
        .get_all_instruments_for_quote_asset(does_not_exists_quote_asset)
        .await;
    assert!(result.is_ok());
    let instruments = result.unwrap();
    assert!(instruments.is_empty());
    let len = instruments.len();
    assert_eq!(len, 0);

    // Test get_all_instruments_for_exchange - success case
    let exists_exchange = "bbit";
    let result = mddb_client
        .get_all_instruments_for_exchange(exists_exchange)
        .await;
    assert!(result.is_ok());
    let instruments = result.unwrap();
    assert!(!instruments.is_empty());
    let len = instruments.len();
    assert!(len > 1);

    // Test get_all_instruments_for_exchange - fail case
    let does_not_exists_exchange = "zztopxyz_non_exist";
    let result = mddb_client
        .get_all_instruments_for_exchange(does_not_exists_exchange)
        .await;
    assert!(result.is_ok());
    let instruments = result.unwrap();
    assert!(instruments.is_empty());
    let len = instruments.len();
    assert_eq!(len, 0);

    // Test get_all_instruments_for_base_asset_and_exchange - success case
    let exists_exchange = "bbit";
    let exists_base_asset = "btc";
    let result = mddb_client
        .get_all_instruments_for_base_asset_and_exchange(exists_exchange, exists_base_asset)
        .await;
    assert!(result.is_ok());
    let instruments = result.unwrap();
    assert!(!instruments.is_empty());
    let len = instruments.len();
    assert!(len > 1);

    // Test get_all_instruments_for_base_asset_and_exchange - fail case
    let does_not_exists_exchange = "zztopxyz_non_exist";
    let does_not_exists_base_asset = "zztopxyz_non_exist";
    let result = mddb_client
        .get_all_instruments_for_base_asset_and_exchange(
            does_not_exists_exchange,
            does_not_exists_base_asset,
        )
        .await;
    assert!(result.is_ok());
    let instruments = result.unwrap();
    assert!(instruments.is_empty());
    let len = instruments.len();
    assert_eq!(len, 0);

    // Test get_all_instruments_for_quote_asset_and_exchange - success case
    let exists_exchange = "bbit";
    let exists_quote_asset = "usdt";
    let result = mddb_client
        .get_all_instruments_for_quote_asset_and_exchange(exists_exchange, exists_quote_asset)
        .await;
    assert!(result.is_ok());
    let instruments = result.unwrap();
    assert!(!instruments.is_empty());
    let len = instruments.len();
    assert!(len > 1);

    // Test get_all_instruments_for_quote_asset_and_exchange - fail case
    let does_not_exists_exchange = "zztopxyz_non_exist";
    let does_not_exists_quote_asset = "zztopxyz_non_exist";
    let result = mddb_client
        .get_all_instruments_for_quote_asset_and_exchange(
            does_not_exists_exchange,
            does_not_exists_quote_asset,
        )
        .await;
    assert!(result.is_ok());
    let instruments = result.unwrap();
    assert!(instruments.is_empty());
    let len = instruments.len();
    assert_eq!(len, 0);

    // Test get_all_instruments_for_base_quote_asset_and_exchange - success case
    let exists_exchange = "bbit";
    let exists_base_asset = "btc";
    let exists_quote_asset = "usd";
    let result = mddb_client
        .get_all_instruments_for_base_quote_asset_and_exchange(
            exists_exchange,
            exists_base_asset,
            exists_quote_asset,
        )
        .await;
    assert!(result.is_ok());
    let instruments = result.unwrap();
    assert!(!instruments.is_empty());
    let len = instruments.len();
    assert_eq!(len, 1); // There should be just one one pair of BTCUSD

    // Test get_all_instruments_for_base_quote_asset_and_exchange - fail case
    let does_not_exists_exchange = "zztopxyz_non_exist";
    let does_not_exists_base_asset = "zztopxyz_non_exist";
    let does_not_exists_quote_asset = "zztopxyz_non_exist";
    let result = mddb_client
        .get_all_instruments_for_base_quote_asset_and_exchange(
            does_not_exists_exchange,
            does_not_exists_base_asset,
            does_not_exists_quote_asset,
        )
        .await;
    assert!(result.is_ok());
    let instruments = result.unwrap();
    assert!(instruments.is_empty());
    let len = instruments.len();
    assert_eq!(len, 0);

    // Test lookup_instrument_id_by_exchange_pair_code - success case
    let exists_instrument_exchange_pair_code = "BTCUSD";
    let result = mddb_client
        .lookup_instrument_id_by_exchange_pair_code(exists_instrument_exchange_pair_code)
        .await;
    assert!(result.is_ok());
    let instrument_id = result.unwrap().unwrap();
    assert_eq!(instrument_id, "bbit_perpetual-future_btc_usd");

    // Test lookup_instrument_id_by_exchange_pair_code - fail case
    let does_not_exists_instrument_exchange_pair_code = "zztopxyz_non_exist";
    let result = mddb_client
        .lookup_instrument_id_by_exchange_pair_code(does_not_exists_instrument_exchange_pair_code)
        .await;
    assert!(result.is_ok());
    let instrument_id = result.unwrap();
    assert!(instrument_id.is_none());

    // Test lookup_instrument_id_by_figi - success case
    let exists_figi = "KKG00000V2R0";
    let result = mddb_client.lookup_instrument_id_by_figi(exists_figi).await;
    assert!(result.is_ok());
    let instrument_id = result.unwrap().unwrap();
    assert_eq!(instrument_id, "bbsp_spot_btc_usdt");

    // Test lookup_instrument_id_by_figi - fail case
    let does_not_exists_figi = "zztopxyz_non_exist";
    let result = mddb_client
        .lookup_instrument_id_by_figi(does_not_exists_figi)
        .await;
    assert!(result.is_ok());
    let instrument_id = result.unwrap();
    assert!(instrument_id.is_none());

    // Test lookup_instrument_id_by_pair_figi
    let exists_pair_figi = "KKG000000SC3";
    let result = mddb_client
        .lookup_instrument_id_by_pair_figi(exists_pair_figi)
        .await;
    assert!(result.is_ok());
    let instrument_id = result.unwrap().unwrap();
    assert_eq!(instrument_id, "bbsp_spot_btc_usdt");

    // Test lookup_instrument_id_by_pair_figi
    let does_not_exists_pair_figi = "zztopxyz_non_exist";
    let result = mddb_client
        .lookup_instrument_id_by_pair_figi(does_not_exists_pair_figi)
        .await;
    assert!(result.is_ok());
    let instrument_id = result.unwrap();
    assert!(instrument_id.is_none());
}
