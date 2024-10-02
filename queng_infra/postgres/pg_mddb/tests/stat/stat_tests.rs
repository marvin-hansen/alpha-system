use common_metadata::prelude::MetaStats;
use container_specs_postgres::postgres_db_container_config;
use diesel::Connection;
use docker_utils::prelude::DockerUtil;
use pg_mddb::prelude::Stat;
use postgres_migrations::prelude::{get_or_wait_for_postgres_connection, DB_TEST_URL};

fn get_test_meta_stats() -> MetaStats {
    let download_timestamp = "2023-10-01T12:00:00Z".to_string();
    let hash = "abc123".to_string();
    let number_assets = 100;
    let number_exchanges = 10;
    let number_instruments = 50;

    MetaStats::new(
        download_timestamp.clone(),
        hash.clone(),
        number_assets,
        number_exchanges,
        number_instruments,
    )
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
async fn test_create_stat() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let meta_stats = MetaStats::new("timestamp".to_string(), "hash".to_string(), 10, 5, 15);
    let result = Stat::create(conn, meta_stats);

    assert!(result.is_ok());
    let created_stat = result.unwrap();
    assert_eq!(created_stat.hash(), "hash");
    assert_eq!(created_stat.download_timestamp(), "timestamp");
    assert_eq!(created_stat.number_assets(), 10);
    assert_eq!(created_stat.number_exchanges(), 5);
    assert_eq!(created_stat.number_instruments(), 15);
}

#[tokio::test]
async fn test_create_stat_err() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let meta_stats = MetaStats::new("timestamp".to_string(), "hash".to_string(), 10, 5, 15);
    let result = Stat::create(conn, meta_stats);

    assert!(result.is_ok());

    let meta_stats = MetaStats::new("timestamp".to_string(), "hash".to_string(), 10, 5, 15);
    let result = Stat::create(conn, meta_stats);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_stat_collection_success() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let meta_stats_collection = vec![MetaStats::new(
        "timestamp1".to_string(),
        "hash1".to_string(),
        10,
        5,
        15,
    )];
    let result = Stat::create_stat_collection(conn, meta_stats_collection);

    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_create_stat_collection_empty() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let meta_stats_collection = vec![];
    let result = Stat::create_stat_collection(conn, meta_stats_collection);

    assert!(result.is_err());
}

#[tokio::test]
async fn test_count_stat() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let test_data = get_test_meta_stats();
    let result = Stat::create(conn, test_data);
    assert!(result.is_ok());

    let result = Stat::count(conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

//
// #[tokio::test]
// async fn test_count_multiple_stat_entries() {
//     let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
//     assert!(connection.is_ok());
//
//     let conn = &mut connection.unwrap();
//     conn.begin_test_transaction().expect("Failed to begin test transaction");
//
//     let result = pg_mddb::run_metadb_migration(conn);
//     assert!(result.is_ok());
//
//     // Insert multiple stat entries
//     for _ in 0..5 {
//         let test_data = get_test_meta_stats();
//         let result = Stat::create(conn, test_data);
//         assert!(result.is_ok());
//     }
//
//     let result = count(conn);
//     assert!(result.is_ok());
//     assert_eq!(result.unwrap(), 5);
// }

#[tokio::test]
async fn test_count_no_stat_entries() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let result = Stat::count(conn);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[tokio::test]
async fn test_check_if_stat_id_exists_returns_true() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let test_data = get_test_meta_stats();
    let result = Stat::create(conn, test_data);
    assert!(result.is_ok());

    let result = Stat::check_if_stat_id_exists(conn, 0);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_check_if_stat_id_exists_returns_false() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let result = Stat::check_if_stat_id_exists(conn, 42);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[tokio::test]
async fn test_read_stat_entry() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let test_data = get_test_meta_stats();
    let result = Stat::create(conn, test_data);
    assert!(result.is_ok());

    let result = Stat::read(conn, 0);
    assert!(result.is_ok());

    let actual = result.unwrap();
    assert_eq!(actual.hash(), "abc123");
    assert_eq!(actual.download_timestamp(), "2023-10-01T12:00:00Z");
    assert_eq!(actual.number_assets(), 100);
    assert_eq!(actual.number_exchanges(), 10);
    assert_eq!(actual.number_instruments(), 50);
}

#[tokio::test]
async fn test_read_non_existent_stat_entry() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let result = Stat::read(conn, 42);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_read_all_stats() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let test_data = get_test_meta_stats();
    let result = Stat::create(conn, test_data);
    assert!(result.is_ok());

    let result = Stat::read_all(conn);
    assert!(result.is_ok());

    let actual = result.unwrap();
    assert_eq!(actual.len(), 1);
}

#[tokio::test]
async fn test_read_all_stats_empty() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let result = Stat::read_all(conn);
    assert!(result.is_ok());

    let actual = result.unwrap();
    assert_eq!(actual.len(), 0);
}

#[tokio::test]
async fn test_delete_stat() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    let test_data = get_test_meta_stats();
    let result = Stat::create(conn, test_data);
    assert!(result.is_ok());

    // Call the delete method
    let stat_id = 0;
    let result = Stat::delete(conn, stat_id);

    // Assert the result
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1); // Assuming 1 row is deleted for the provided stat_id
}

#[tokio::test]
async fn test_delete_stat_non_existent_id() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = pg_mddb::run_mddb_migration(conn);
    assert!(result.is_ok());

    // Call the delete method
    let stat_id = 99;
    let result = Stat::delete(conn, stat_id);

    // Assert the result
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0); // Zero row is deleted since the id does not exist
}
