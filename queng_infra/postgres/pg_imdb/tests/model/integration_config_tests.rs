use common_ims::{
    ExchangeID, ImsIntegrationType, IntegrationConfig as CommonIntegrationConfig,
    IntegrationMessageConfig,
};
use container_specs_postgres::postgres_db_container_config;
use diesel::Connection;
use docker_utils::prelude::DockerUtil;
use pg_imdb::model::integration_config::IntegrationConfig;
use postgres_migrations::{get_or_wait_for_postgres_connection, DB_TEST_URL};

//
// Somehow tests seem to be executed or sorted in alphabetical order,
// so make sure that the setup is on top of the stack.
#[tokio::test]
async fn all_setup() {
    let env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres container
    let container_config = postgres_db_container_config();
    let result = env.get_or_start_container_config(&container_config); // dbg!(&result);
                                                                       // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_schema_migration() {
    // Create a new connection
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_integration_config_success() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    let config = CommonIntegrationConfig::new(
        "test-id".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
    );

    let result = IntegrationConfig::create(conn, &config);
    dbg!(&result);
    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(result.integration_id(), "test-id".to_string());
    assert_eq!(result.ims_integration_type(), ImsIntegrationType::Data);
    assert_eq!(result.exchange_id(), ExchangeID::Binance);
}
#[tokio::test]
async fn test_insert_integration_config_collection() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    let configs = vec![
        CommonIntegrationConfig::new(
            "test-id-1".to_string(),
            1,
            ImsIntegrationType::Data,
            ExchangeID::Binance,
            IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
        ),
        CommonIntegrationConfig::new(
            "test-id-2".to_string(),
            1,
            ImsIntegrationType::Data,
            ExchangeID::Kraken,
            IntegrationMessageConfig::new(1, 1, ExchangeID::Kraken),
        ),
    ];

    let result = IntegrationConfig::insert_integration_config_collection(conn, &configs);
    assert!(result.is_ok());

    // Verify configs were inserted
    let insert_count = result.unwrap();
    assert_eq!(insert_count, 2);
}

#[tokio::test]
async fn test_insert_integration_config_collection_empty() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    // dbg!(&connection);
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    // Start a new test transaction
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    let configs: Vec<CommonIntegrationConfig> = vec![];
    let result = IntegrationConfig::insert_integration_config_collection(conn, &configs);

    // Verify configs were inserted
    let insert_count = result.unwrap();
    assert_eq!(insert_count, 0);
}

#[tokio::test]
async fn test_count_integration_config() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Initial count should be 0
    let result = IntegrationConfig::count(conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    // Create test config
    let config = CommonIntegrationConfig::new(
        "test-id".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
    );

    let result = IntegrationConfig::create(conn, &config);
    dbg!(&result);
    assert!(result.is_ok());

    // Count should now be 1
    let result = IntegrationConfig::count(conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);

    // Add another config
    let config2 = CommonIntegrationConfig::new(
        "test-id-2".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Kraken,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Kraken),
    );

    let create_result = IntegrationConfig::create(conn, &config2);
    assert!(create_result.is_ok());

    // Count should now be 2
    let result = IntegrationConfig::count(conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);
}

#[tokio::test]
async fn test_check_if_integration_config_online() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Create test configs with different online states
    let mut online_config = CommonIntegrationConfig::new(
        "online-config".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
    );
    online_config.set_online();

    let offline_config = CommonIntegrationConfig::new(
        "offline-config".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Kraken,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Kraken),
    );

    // Insert configs
    IntegrationConfig::create(conn, &online_config).unwrap();
    IntegrationConfig::create(conn, &offline_config).unwrap();

    // Test online config
    let result =
        IntegrationConfig::check_if_integration_config_online(conn, "online-config".to_string());
    assert!(result.is_ok());
    assert!(result.unwrap());

    // Test offline config
    let result =
        IntegrationConfig::check_if_integration_config_online(conn, "offline-config".to_string());
    assert!(result.is_ok());
    assert!(!result.unwrap());

    // Test non-existent config
    let result =
        IntegrationConfig::check_if_integration_config_online(conn, "nonexistent".to_string());
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_integration_config() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Create test config
    let test_config = CommonIntegrationConfig::new(
        "test-integration".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
    );

    // Insert config
    let create_result = IntegrationConfig::create(conn, &test_config);
    assert!(create_result.is_ok());

    // Test successful retrieval
    let result = IntegrationConfig::get_integration_config(conn, "test-integration".to_string());
    assert!(result.is_ok());

    let res = result.unwrap();
    assert!(res.is_some());
    let retrieved_config = res.unwrap();
    assert_eq!(
        retrieved_config.integration_id(),
        test_config.integration_id()
    );
    assert_eq!(
        retrieved_config.integration_version(),
        test_config.integration_version()
    );
    assert_eq!(
        retrieved_config.ims_integration_type(),
        test_config.ims_integration_type()
    );
    assert_eq!(retrieved_config.exchange_id(), test_config.exchange_id());
    assert_eq!(
        retrieved_config.integration_message_config(),
        test_config.integration_message_config()
    );

    // Test non-existent config
    let result = IntegrationConfig::get_integration_config(conn, "nonexistent".to_string());
    assert!(result.is_ok());

    let res = result.unwrap();
    assert!(res.is_none());
}

#[tokio::test]
async fn test_get_all_integration_configs() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Test empty state
    let configs = IntegrationConfig::get_all_integration_configs(conn).unwrap();
    assert_eq!(configs.len(), 0);

    // Create test configs
    let test_configs = vec![
        CommonIntegrationConfig::new(
            "config-1".to_string(),
            1,
            ImsIntegrationType::Data,
            ExchangeID::Binance,
            IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
        ),
        CommonIntegrationConfig::new(
            "config-2".to_string(),
            2,
            ImsIntegrationType::Execution,
            ExchangeID::Kraken,
            IntegrationMessageConfig::new(2, 1, ExchangeID::Kraken),
        ),
        CommonIntegrationConfig::new(
            "config-3".to_string(),
            3,
            ImsIntegrationType::Data,
            ExchangeID::Binance,
            IntegrationMessageConfig::new(3, 1, ExchangeID::Binance),
        ),
    ];

    // Insert configs
    IntegrationConfig::insert_integration_config_collection(conn, &test_configs).unwrap();

    // Test retrieval
    let stored_configs = IntegrationConfig::get_all_integration_configs(conn).unwrap();
    assert_eq!(stored_configs.len(), 3);

    // Verify config contents
    for (stored, original) in stored_configs.iter().zip(test_configs.iter()) {
        assert_eq!(stored.integration_id(), original.integration_id());
        assert_eq!(stored.integration_version(), original.integration_version());
        assert_eq!(
            stored.ims_integration_type(),
            original.ims_integration_type()
        );
        assert_eq!(stored.exchange_id(), original.exchange_id());
        assert_eq!(
            stored.integration_message_config(),
            original.integration_message_config()
        );
    }
}

#[tokio::test]
async fn test_get_configs_for_valid_exchange() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Test empty state;  result must be zero
    let configs = IntegrationConfig::get_all_integration_configs(conn).unwrap();
    assert_eq!(configs.len(), 0);

    // Create test configs;
    let test_configs = vec![
        CommonIntegrationConfig::new(
            "config-1".to_string(),
            1,
            ImsIntegrationType::Data,
            ExchangeID::Binance,
            IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
        ),
        CommonIntegrationConfig::new(
            "config-2".to_string(),
            2,
            ImsIntegrationType::Execution,
            ExchangeID::Kraken,
            IntegrationMessageConfig::new(2, 1, ExchangeID::Kraken),
        ),
        CommonIntegrationConfig::new(
            "config-3".to_string(),
            3,
            ImsIntegrationType::Data,
            ExchangeID::Binance,
            IntegrationMessageConfig::new(3, 1, ExchangeID::Binance),
        ),
    ];

    // Insert configs
    IntegrationConfig::insert_integration_config_collection(conn, &test_configs).unwrap();

    // Test retrieval
    let stored_configs = IntegrationConfig::get_all_integration_configs(conn).unwrap();
    assert_eq!(stored_configs.len(), 3);

    let param_exchange_id = ExchangeID::Binance as i32;
    let results =
        IntegrationConfig::get_all_integration_configs_by_exchange(conn, param_exchange_id);

    assert!(results.is_ok());
    let res = results.unwrap();
    assert_eq!(res.len(), 2);
}

#[tokio::test]
async fn test_get_all_online_integration_configs() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Test empty state
    let configs = IntegrationConfig::get_all_online_integration_configs(conn).unwrap();
    assert_eq!(configs.len(), 0);

    // Create mixed online/offline configs
    let mut online_config1 = CommonIntegrationConfig::new(
        "online-1".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
    );
    online_config1.set_online();

    let mut online_config2 = CommonIntegrationConfig::new(
        "online-2".to_string(),
        2,
        ImsIntegrationType::Execution,
        ExchangeID::Kraken,
        IntegrationMessageConfig::new(2, 1, ExchangeID::Kraken),
    );
    online_config2.set_online();

    let offline_config = CommonIntegrationConfig::new(
        "offline-1".to_string(),
        3,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(3, 1, ExchangeID::Binance),
    );

    // Insert configs
    IntegrationConfig::create(conn, &online_config1).unwrap();
    IntegrationConfig::create(conn, &online_config2).unwrap();
    IntegrationConfig::create(conn, &offline_config).unwrap();

    // Test retrieval
    let online_configs = IntegrationConfig::get_all_online_integration_configs(conn).unwrap();
    assert_eq!(online_configs.len(), 2);

    // Verify all returned configs are online
    assert!(online_configs.iter().all(|c| c.online()));

    // Verify correct configs returned
    let config_ids: Vec<String> = online_configs
        .iter()
        .map(|c| c.integration_id().to_string())
        .collect();
    assert!(config_ids.contains(&"online-1".to_string()));
    assert!(config_ids.contains(&"online-2".to_string()));
    assert!(!config_ids.contains(&"offline-1".to_string()));
}

#[tokio::test]
async fn test_get_all_online_integration_configs_empty() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Create only offline configs
    let offline_config = CommonIntegrationConfig::new(
        "offline-1".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
    );

    IntegrationConfig::create(conn, &offline_config).unwrap();

    // Test retrieval
    let online_configs = IntegrationConfig::get_all_online_integration_configs(conn).unwrap();
    assert_eq!(online_configs.len(), 0);
}

#[tokio::test]
async fn test_get_all_offline_integration_configs() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Test empty state
    let configs = IntegrationConfig::get_all_offline_integration_configs(conn).unwrap();
    assert_eq!(configs.len(), 0);

    // Create mixed online/offline configs
    let mut online_config = CommonIntegrationConfig::new(
        "online-1".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
    );
    online_config.set_online();

    let offline_config1 = CommonIntegrationConfig::new(
        "offline-1".to_string(),
        2,
        ImsIntegrationType::Execution,
        ExchangeID::Kraken,
        IntegrationMessageConfig::new(2, 1, ExchangeID::Kraken),
    );

    let offline_config2 = CommonIntegrationConfig::new(
        "offline-2".to_string(),
        3,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(3, 1, ExchangeID::Binance),
    );

    // Insert configs
    IntegrationConfig::create(conn, &online_config).unwrap();
    IntegrationConfig::create(conn, &offline_config1).unwrap();
    IntegrationConfig::create(conn, &offline_config2).unwrap();

    // Test retrieval
    let offline_configs = IntegrationConfig::get_all_offline_integration_configs(conn).unwrap();
    assert_eq!(offline_configs.len(), 2);

    // Verify all returned configs are offline
    assert!(offline_configs.iter().all(|c| !c.online()));

    // Verify correct configs returned
    let config_ids: Vec<String> = offline_configs
        .iter()
        .map(|c| c.integration_id().to_string())
        .collect();
    assert!(config_ids.contains(&"offline-1".to_string()));
    assert!(config_ids.contains(&"offline-2".to_string()));
    assert!(!config_ids.contains(&"online-1".to_string()));
}

#[tokio::test]
async fn test_get_all_offline_integration_configs_empty() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Create only online configs
    let mut online_config = CommonIntegrationConfig::new(
        "online-1".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
    );
    online_config.set_online();

    IntegrationConfig::create(conn, &online_config).unwrap();

    // Test retrieval
    let offline_configs = IntegrationConfig::get_all_offline_integration_configs(conn).unwrap();
    assert_eq!(offline_configs.len(), 0);
}

#[tokio::test]
async fn test_update_integration_config() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Create initial config
    let mut initial_config = CommonIntegrationConfig::new(
        "test-integration".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
    );
    initial_config.set_online();

    IntegrationConfig::create(conn, &initial_config).unwrap();

    // Create updated config
    let mut updated_config = CommonIntegrationConfig::new(
        "test-integration".to_string(), // Same ID
        2,                              // New version
        ImsIntegrationType::Execution,  // Changed type
        ExchangeID::Kraken,             // Changed exchange
        IntegrationMessageConfig::new(2, 2, ExchangeID::Kraken),
    );
    updated_config.set_online(); // Changed online status

    // Perform update
    let result =
        IntegrationConfig::update_integration_config(conn, updated_config.clone()).unwrap();

    assert_eq!(result, 1)

    // Verify updated fields
    // assert_eq!(result.integration_id(), updated_config.integration_id());
    // assert_eq!(
    //     result.integration_version(),
    //     updated_config.integration_version()
    // );
    // assert_eq!(
    //     result.ims_integration_type(),
    //     updated_config.ims_integration_type()
    // );
    // assert_eq!(result.exchange_id(), updated_config.exchange_id());
    // assert_eq!(
    //     result.integration_message_config(),
    //     updated_config.integration_message_config()
    // );
    // assert_eq!(result.online(), updated_config.online());
}

#[tokio::test]
async fn test_update_integration_config_not_found() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    let config = CommonIntegrationConfig::new(
        "nonexistent".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
    );

    let result = IntegrationConfig::update_integration_config(conn, config);
    assert!(result.is_ok());
    let updated = result.unwrap();
    assert_eq!(updated, 0)
}

#[tokio::test]
async fn test_update_integration_config_partial() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Create initial config
    let initial_config = CommonIntegrationConfig::new(
        "test-partial".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
    );

    IntegrationConfig::create(conn, &initial_config).unwrap();

    // Update only version and online status
    let mut partial_update = initial_config.clone();
    partial_update.set_online();

    let result =
        IntegrationConfig::update_integration_config(conn, partial_update.clone()).unwrap();

    assert_eq!(result, 1)

    // Verify updated and unchanged fields
    // assert!(result.online());
    // assert_eq!(
    //     result.ims_integration_type(),
    //     initial_config.ims_integration_type()
    // );
    // assert_eq!(result.exchange_id(), initial_config.exchange_id());
}

#[tokio::test]
async fn test_delete_integration_config() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Create test config
    let config = CommonIntegrationConfig::new(
        "test-delete".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
    );

    IntegrationConfig::create(conn, &config).unwrap();

    // Verify config exists
    let exists = IntegrationConfig::get_integration_config(conn, "test-delete".to_string()).is_ok();
    assert!(exists);

    // Delete config
    let result =
        IntegrationConfig::delete_integration_config(conn, "test-delete".to_string()).unwrap();
    assert_eq!(result, 1); // One row affected

    // Verify config no longer exists
    let get_result = IntegrationConfig::get_integration_config(conn, "test-delete".to_string());
    assert!(get_result.is_ok());
    assert!(get_result.unwrap().is_none());
}

#[tokio::test]
async fn test_delete_nonexistent_integration_config() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Attempt to delete non-existent config
    let result =
        IntegrationConfig::delete_integration_config(conn, "nonexistent".to_string()).unwrap();
    assert_eq!(result, 0); // No rows affected
}

#[tokio::test]
async fn test_delete_multiple_integration_configs() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_imdb::run_imdb_db_migration(conn);
    assert!(result.is_ok());

    // Create multiple configs
    let configs = vec![
        CommonIntegrationConfig::new(
            "config-1".to_string(),
            1,
            ImsIntegrationType::Data,
            ExchangeID::Binance,
            IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
        ),
        CommonIntegrationConfig::new(
            "config-2".to_string(),
            2,
            ImsIntegrationType::Execution,
            ExchangeID::Kraken,
            IntegrationMessageConfig::new(2, 1, ExchangeID::Kraken),
        ),
    ];

    for config in configs {
        IntegrationConfig::create(conn, &config).unwrap();
    }

    // Delete configs one by one
    let result1 =
        IntegrationConfig::delete_integration_config(conn, "config-1".to_string()).unwrap();
    assert_eq!(result1, 1);

    let result2 =
        IntegrationConfig::delete_integration_config(conn, "config-2".to_string()).unwrap();
    assert_eq!(result2, 1);

    // Verify all configs are deleted
    let remaining = IntegrationConfig::get_all_integration_configs(conn).unwrap();
    assert_eq!(remaining.len(), 0);
}
