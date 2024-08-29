use common_database::prelude::PostgresDBSchema;
use diesel::Connection;
use pg_cmdb::model::portfolio::Portfolio;
use postgres_test_utils::prelude::{get_test_portfolio, postgres_schema_setup};
use postgres_test_utils::{postgres_connection, DB_TEST_URL};

#[tokio::test]
async fn test_delete_portfolio() {
    postgres_schema_setup(PostgresDBSchema::CMDB, DB_TEST_URL)
        .await
        .expect("FAILED  to setup CMDB schema");

    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;

    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let portfolio = get_test_portfolio();
    let result = Portfolio::create(conn, &portfolio);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Portfolio::check_if_portfolio_id_exists(conn, 1);
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());

    let result = Portfolio::delete(conn, 1);
    dbg!(&result);
    assert!(result.is_ok());

    let result = Portfolio::check_if_portfolio_id_exists(conn, 1);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}
