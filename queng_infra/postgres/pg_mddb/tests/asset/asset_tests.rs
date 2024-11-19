use container_specs_postgres::postgres_db_container_config;
use diesel::Connection;
use docker_utils::DockerUtil;
use pg_mddb::Asset;
use postgres_migrations::{get_or_wait_for_postgres_connection, DB_TEST_URL};

fn get_test_asset() -> Asset {
    Asset {
        asset_code: "test_asset_code".to_string(),
        asset_name: "test_asset_name".to_string(),
        asset_class: "test_asset_class".to_string(),
        asset_classes: vec![],
        asset_figi: None,
        asset_hash: "test_asset_hash".to_string(),
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
    dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_count() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let count = Asset::count(conn);
    assert!(count.is_ok());
    assert_eq!(count.unwrap(), 0);

    let asset = get_test_asset().to_meta_asset();
    let result = Asset::create_asset(conn, asset);
    assert!(result.is_ok());
    let count = Asset::count(conn);
    assert!(count.is_ok());
    assert_eq!(count.unwrap(), 1);
}

#[tokio::test]
async fn test_create() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let asset = get_test_asset().to_meta_asset();
    let result = Asset::create_asset(conn, asset);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_error() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let asset = get_test_asset().to_meta_asset();
    let result = Asset::create_asset(conn, asset.clone());
    assert!(result.is_ok());

    // Simulate an error scenario by trying to create the same asset again with the same code (primary key)
    let result = Asset::create_asset(conn, asset);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_read() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let asset = get_test_asset().to_meta_asset();
    let result = Asset::create_asset(conn, asset);
    assert!(result.is_ok());

    let result = Asset::read(conn, "test_asset_code".to_string());
    assert!(result.is_ok());

    let asset = result.unwrap().unwrap();
    assert_eq!(asset.code, "test_asset_code".to_string());
}

#[tokio::test]
async fn test_read_error() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    // Simulate an error scenario
    let result = Asset::read(conn, "DoesNotExists".to_string());
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_delete() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_migration(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let asset = get_test_asset().to_meta_asset();
    let result = Asset::create_asset(conn, asset);
    assert!(result.is_ok());

    let result = Asset::delete(conn, "test_asset_code".to_string());
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_error() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Asset::delete(conn, "DoesNotExists".to_string());
    dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
