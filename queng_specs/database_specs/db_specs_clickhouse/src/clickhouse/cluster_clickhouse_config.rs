use common_database::ClickHouseConfig;

#[must_use]
pub fn get_cluster_db_config() -> ClickHouseConfig {
    ClickHouseConfig::new(
        "http://clickhouse.default.svc.cluster.local".to_string(),
        8123,
        "username".to_string(),
        "password".to_string(),
        "metadata".to_string(),
    )
}
