use common_database::prelude::PostgresDBSchema;
use diesel::Connection;
use pg_cmdb::model::portfolio::Portfolio;
use postgres_test_utils::prelude::{get_test_portfolio, postgres_schema_setup};
use postgres_test_utils::{postgres_connection, DB_TEST_URL};

#[tokio::test]
async fn test_count_portfolio() {
    postgres_schema_setup(PostgresDBSchema::CMDB, DB_TEST_URL)
        .await
        .expect("FAILED  to setup CMDB schema");

    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = Portfolio::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let count = result.unwrap();
    assert_eq!(count, 0);

    let portfolio = get_test_portfolio();
    let result = Portfolio::create(conn, &portfolio);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Portfolio::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let count = result.unwrap();
    assert_eq!(count, 1);
}
