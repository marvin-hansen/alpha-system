/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use container_specs_clickhouse::clickhouse_container_config;

#[test]
fn test_clickhouse_container_config() {
    let config = clickhouse_container_config();

    assert_eq!(config.name(), "clickhouse");
    assert_eq!(
        config.container_image(),
        "clickhouse/clickhouse-server:24.6.1"
    );

    assert_eq!(config.url(), "0.0.0.0");
    assert_eq!(config.connection_port(), 9000);
    assert_eq!(config.platform(), None);
    assert!(config.reuse_container());
    assert!(config.keep_configuration());
}
