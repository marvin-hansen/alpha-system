use db_specs::clickhouse_db::local::clickhouse_local_config;

#[test]
fn test_get_specs_db_config() {
    let config = clickhouse_local_config::get_local_specs_db_config();

    assert_eq!(config.url(), "127.0.0.1");
    assert_eq!(config.port(), 9000);
    assert_eq!(config.username(), "default");
    assert_eq!(config.password(), "");
    assert_eq!(config.database(), "specs");
}

#[test]
fn test_get_metadata_db_config() {
    let config = clickhouse_local_config::get_local_metadata_db_config();

    assert_eq!(config.url(), "127.0.0.1");
    assert_eq!(config.port(), 9000);
    assert_eq!(config.username(), "default");
    assert_eq!(config.password(), "");
    assert_eq!(config.database(), "metadata");
}
