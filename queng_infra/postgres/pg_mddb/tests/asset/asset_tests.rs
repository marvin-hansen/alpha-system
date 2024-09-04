use diesel::Connection;
use env_utils::EnvUtil;
use pg_mddb::prelude::Asset;
use postgres_test_utils::{get_or_wait_for_postgres_connection, DB_TEST_URL};

// Somehow tests seem to be executed or sorted in alphabetical order, so make sure that the
// setup is on top of the stack.
#[tokio::test]
async fn all_setup() {
    let env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    // Start or reuse a test postgres container
    let result = env.setup_container_postgres_db().await;
    // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_error() {
    let connection = get_or_wait_for_postgres_connection(DB_TEST_URL, None).await;
    assert!(connection.is_ok());
    let conn = &mut connection.unwrap();
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");
    let result = pg_mddb::run_mddb_db_migration(conn);
    dbg!(&result);
    assert!(result.is_ok());

    // Docker version 27.1.2, build d01f264
    // test test_delete_error ... FAILED
    // test all_setup ... ok
    //
    // failures:
    //
    // ---- test_delete_error stdout ----
    // [pg_smdb]: Error migrating database: Failed to run 2024-08-23-093731_mddb with:
    // Received an empty query[queng_infra/postgres/pg_mddb/tests/asset/asset_tests.rs:24:5] &result = Err(
    //     QueryError(
    //         DieselMigrationName {
    //             name: "2024-08-23-093731_mddb",
    //             version: MigrationVersion(
    //                 "20240823093731",
    //             ),
    //         },
    //         DatabaseError(
    //             Unknown,
    //             "Received an empty query",
    //         ),
    //     ),
    // )
    // thread 'test_delete_error' panicked at queng_infra/postgres/pg_mddb/tests/asset/asset_tests.rs:25:5:
    // assertion failed: result.is_ok()
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    let result = Asset::delete(conn, "DoesNotExists".to_string());
    assert!(result.is_err());
}
