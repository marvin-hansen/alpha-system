use db_specs_clickhouse::cluster_clickhouse_config;

#[test]
fn test_get_cluster_metadata_db_config() {
    let config = cluster_clickhouse_config::get_cluster_db_config();

    assert_eq!(config.url(), "http://clickhouse.default.svc.cluster.local");
    assert_eq!(config.port(), 8123);
    assert_eq!(config.username(), "username");
    assert_eq!(config.password(), "password");
    assert_eq!(config.database(), "metadata");
}
