use common_database::prelude::PostgresDBSchema;
use diesel::Connection;
use pg_cmdb::model::portfolio::Portfolio;
use postgres_test_utils::prelude::postgres_schema_setup;
use postgres_test_utils::{postgres_connection, DB_TEST_URL};

#[tokio::test]
async fn test_read_all_portfolio_empty() {
    postgres_schema_setup(PostgresDBSchema::CMDB, DB_TEST_URL)
        .await
        .expect("FAILED  to setup CMDB schema");

    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = Portfolio::read_all(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let all_portfolios = result.unwrap();
    assert_eq!(all_portfolios.len(), 0);
}
