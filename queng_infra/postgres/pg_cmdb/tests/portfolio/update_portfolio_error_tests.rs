use common_database::prelude::PostgresDBSchema;
use common_exchange::prelude::{AccountType, PortfolioConfig as CommonPortfolioConfig};
use diesel::Connection;
use pg_cmdb::model::portfolio::Portfolio;
use postgres_test_utils::prelude::postgres_schema_setup;
use postgres_test_utils::{postgres_connection, DB_TEST_URL};

#[tokio::test]
async fn test_update_portfolio_error_id_does_not_exist() {
    postgres_schema_setup(PostgresDBSchema::CMDB, DB_TEST_URL)
        .await
        .expect("FAILED  to setup CMDB schema");

    let mut connection = postgres_connection(DB_TEST_URL).await;
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let update = CommonPortfolioConfig::new(
        1,
        "Updated Portfolio".to_string(),
        AccountType::Spot,
        "67890".to_string(),
        "EUR".to_string(),
        1000.0,
        500.0,
        20.0,
        Vec::new(),
        3.0,
        10.0,
        1000.0,
        2000.0,
        50.0,
        100.0,
    );

    let result = Portfolio::update(conn, 1, &update);
    assert!(result.is_err());
}
