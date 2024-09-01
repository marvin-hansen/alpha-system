use common_database::prelude::PostgresDBSchema;
use postgres_test_utils::prelude::*;

#[tokio::test]
async fn test_cmdb_setup() {
    let result = postgres_schema_setup(PostgresDBSchema::PostgresDBSchemaCMDB, DB_TEST_URL).await;
    // dbg!(&result);
    assert!(result.is_ok());
}
