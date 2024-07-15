use db_specs::clickhouse_db::cluster::clickhouse_cluster_config;

#[test]
fn test_get_cluster_specs_db_config() {
    let config = clickhouse_cluster_config::get_cluster_specs_db_config();

    assert_eq!(config.url(), "http://clickhouse.default.svc.cluster.local");
    assert_eq!(config.port(), 8123);
    assert_eq!(config.username(), "username");
    assert_eq!(config.password(), "password");
    assert_eq!(config.database(), "specs");
}

#[test]
fn test_get_cluster_metadata_db_config() {
    let config = clickhouse_cluster_config::get_cluster_metadata_db_config();

    assert_eq!(config.url(), "http://clickhouse.default.svc.cluster.local");
    assert_eq!(config.port(), 8123);
    assert_eq!(config.username(), "username");
    assert_eq!(config.password(), "password");
    assert_eq!(config.database(), "metadata");
}
