use container_specs_postgres::postgres_db_container_config;

//  When you update the Dockertag, also update the postgres.sh script in scripts/ folder
#[test]
fn test_postgres_db_container_config() {
    let config = postgres_db_container_config();

    assert_eq!(config.name(), "postgres");
    assert_eq!(config.image(), "postgres");
    assert_eq!(config.tag(), "17-alpine3.20");
    assert_eq!(config.url(), "0.0.0.0");
    assert_eq!(config.connection_port(), 5432);
    assert!(config.additional_ports().is_none());
    assert!(config.additional_env_vars().is_some());
    assert!(config.platform().is_none());
    assert!(config.reuse_container());
    assert!(config.keep_configuration());
}
