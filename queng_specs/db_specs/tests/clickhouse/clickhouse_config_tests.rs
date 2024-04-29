use db_specs::clickhouse::{get_cluster_db_config, get_local_db_config};

#[test]
fn test_get_local_db_config() {
    let config = get_local_db_config();

    assert_eq!(config.url(), "127.0.0.1");
    assert_eq!(config.port(), 9000);
    assert_eq!(config.username(), "");
    assert_eq!(config.password(), "");
    assert_eq!(config.database(), "default");
}

#[test]
fn test_get_cluster_db_config() {
    let config = get_cluster_db_config();

    assert_eq!(config.url(), "http://clickhouse.default.svc.cluster.local");
    assert_eq!(config.port(), 8123);
    assert_eq!(config.username(), "username");
    assert_eq!(config.password(), "password");
    assert_eq!(config.database(), "default");
}
