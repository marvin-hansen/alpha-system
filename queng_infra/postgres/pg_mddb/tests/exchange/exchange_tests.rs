use common_metadata::prelude::MetaExchange;
use container_specs_postgres::postgres_db_container_config;
use diesel::Connection;
use docker_utils::prelude::DockerUtil;
use pg_mddb::prelude::Exchange;
use postgres_migrations::prelude::{get_or_wait_for_postgres_connection, DB_TEST_URL};

fn get_test_meta_exchange() -> MetaExchange {
    MetaExchange {
        code: "test_exchange_code".to_string(),
        name: "test_exchange_name".to_string(),
        kaiko_legacy_slug: "test_kaiko_legacy_slug".to_string(),
    }
}

// Somehow tests seem to be executed or sorted in alphabetical order, so make sure that the
// setup is on top of the stack.
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
async fn test_migration() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_exchange() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let test_data = get_test_meta_exchange();
    let result = Exchange::create_exchange(conn, test_data);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_exchange_error() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let test_data = get_test_meta_exchange();
    let result = Exchange::create_exchange(conn, test_data);
    assert!(result.is_ok());

    // Simulate an error scenario by trying to create the same exchange again with the same code (primary key)
    let test_data = get_test_meta_exchange();
    let result = Exchange::create_exchange(conn, test_data);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_exchange_collection() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let test_data = Vec::from([get_test_meta_exchange()]);
    let result = Exchange::create_exchange_collection(conn, &test_data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[tokio::test]
async fn test_count_exchanges_with_entries() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let test_data = get_test_meta_exchange();
    let result = Exchange::create_exchange(conn, test_data);
    assert!(result.is_ok());

    let result = Exchange::count(conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[tokio::test]
async fn test_count_exchanges_no_entries() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let result = Exchange::count(conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[tokio::test]
async fn test_check_if_exchange_id_exists_true() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let test_data = get_test_meta_exchange();
    let valid_exchange_id = test_data.code.clone();

    let result = Exchange::create_exchange(conn, test_data);
    assert!(result.is_ok());

    let result = Exchange::check_if_exchange_id_exists(conn, valid_exchange_id);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_check_if_exchange_id_exists_false() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let invalid_exchange_id = "non_existent_exchange_id";
    let result = Exchange::check_if_exchange_id_exists(conn, invalid_exchange_id.to_string());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[tokio::test]
async fn test_update() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let test_data = get_test_meta_exchange();
    Exchange::create_exchange(conn, test_data).expect("Failed to create exchange collection");

    let exchange_id = "test_exchange_code"; // Replace with actual valid ID
    let updated_data = get_test_meta_exchange(); // Replace with actual updated data
    let result = Exchange::update(conn, exchange_id.to_string(), updated_data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[tokio::test]
async fn test_read_valid_exchange_id() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let test_data = get_test_meta_exchange();
    let valid_exchange_id = test_data.code.clone();
    Exchange::create_exchange(conn, test_data.clone()).expect("Failed to create exchange");

    let result = Exchange::read(conn, valid_exchange_id);
    assert!(result.is_ok());
    let actual_exchange = result.unwrap();
    assert_eq!(actual_exchange.name, test_data.name);
    assert_eq!(actual_exchange.code, test_data.code);
}

#[tokio::test]
async fn test_read_all_with_entries() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let test_data1 = get_test_meta_exchange();
    Exchange::create_exchange(conn, test_data1.clone()).expect("Failed to create exchange");

    let result = Exchange::read_all(conn);
    assert!(result.is_ok());
    let exchanges = result.unwrap();
    assert_eq!(exchanges.len(), 1);
}

#[tokio::test]
async fn test_read_all_empty_table() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let result = Exchange::read_all(conn);
    assert!(result.is_ok());
    let exchanges = result.unwrap();
    assert!(exchanges.is_empty());
}

#[tokio::test]
async fn test_read_non_existent_exchange_id() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let invalid_exchange_id = "non_existent_exchange_id";
    let result = Exchange::read(conn, invalid_exchange_id.to_string());
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_error() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let invalid_exchange_id = "invalid_exchange_id";
    let updated_data = get_test_meta_exchange(); // Replace with actual updated data
    let result = Exchange::update(conn, invalid_exchange_id.to_string(), updated_data.clone());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
