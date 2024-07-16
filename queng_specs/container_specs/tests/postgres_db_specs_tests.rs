use container_specs::postgres_db_specs::postgres_db_container_config;

#[test]
fn test_surreal_db_container_config() {
    let config = postgres_db_container_config();

    assert_eq!(config.name(), "surrealdb");
    assert_eq!(config.image(), "surrealdb/surrealdb");
    assert_eq!(config.tag(), "v1.5.4");
    assert_eq!(config.url(), "0.0.0.0");
    assert_eq!(config.connection_port(), 8000);
    assert!(config.additional_ports().is_none());
    assert!(config.platform().is_none());
    assert!(config.reuse_container());
    assert!(config.keep_configuration());
}
