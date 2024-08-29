use common_database::prelude::PostgresDBSchema;
use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_setup() {
    env!("CMDB_MIGRATION_READY");
    std::env::set_var("CMDB_MIGRATION_READY", "False");

    let result = postgres_schema_setup(PostgresDBSchema::CMDB, DB_TEST_URL).await;

    assert!(result.is_ok());
    std::env::set_var("CMDB_MIGRATION_READY", "True");
}
