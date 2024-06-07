use container_specs::clickhouse_container_config::clickhouse_container_config;

#[test]
fn test_clickhouse_container_config() {
    let config = clickhouse_container_config();

    assert_eq!(config.name(), "clickhouse");
    assert_eq!(
        config.container_image(),
        "clickhouse/clickhouse-server:24.3.2"
    );
    assert_eq!(config.url(), "127.0.0.1");
    assert_eq!(config.connection_port(), 9000);
    assert_eq!(config.platform(), None);
    assert!(config.reuse_container());
    assert!(config.reset_configuration());
}
