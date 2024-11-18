use common_env::EnvironmentType;
use db_specs_postgres::get_postgres_config;

#[test]
fn test_get_ci_db_config() {
    let config = get_postgres_config(&EnvironmentType::CI);
    assert_eq!(config.pg_host(), "localhost");
    assert_eq!(config.pg_user(), "postgres");
    assert_eq!(config.pg_password(), "postgres");
    assert_eq!(config.pg_database(), "postgres");
    assert_eq!(config.pg_port(), 5432);
    assert_eq!(config.pg_max_connections(), 5);
}
