use db_specs_postgres::postgres::get_local_db_config;

#[test]
fn test_get_local_db_config() {
    let config = get_local_db_config();
    assert_eq!(config.pg_host(), "localhost");
    assert_eq!(config.pg_user(), "postgres");
    assert_eq!(config.pg_password(), "postgres");
    assert_eq!(config.pg_database(), "postgres");
    assert_eq!(config.pg_port(), 5432);
    assert_eq!(config.pg_max_connections(), 5);
}
