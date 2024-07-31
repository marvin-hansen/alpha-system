use common_database::prelude::PostgresDBConfig;

#[test]
fn test_postgres_db_config() {
    // Setup - create an instance of PostgresDBConfig with known values
    let config = PostgresDBConfig::new(
        "localhost".to_string(),
        "user".to_string(),
        "password".to_string(),
        "test_db".to_string(),
        5432,
        10,
    );

    // Exercise & Verify - test each accessor method
    assert_eq!(config.pg_host(), "localhost");
    assert_eq!(config.pg_user(), "user");
    assert_eq!(config.pg_password(), "password");
    assert_eq!(config.pg_database(), "test_db");
    assert_eq!(config.pg_port(), 5432);
    assert_eq!(config.pg_max_connections(), 10);
}
