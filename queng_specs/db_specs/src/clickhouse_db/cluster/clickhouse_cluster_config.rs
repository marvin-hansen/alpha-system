use common_database::prelude::ClickHouseConfig;

pub fn get_cluster_specs_db_config() -> ClickHouseConfig {
    base_cluster_db_config("specs")
}
pub fn get_cluster_metadata_db_config() -> ClickHouseConfig {
    base_cluster_db_config("metadata")
}

fn base_cluster_db_config(db: &str) -> ClickHouseConfig {
    ClickHouseConfig::new(
        "http://clickhouse.default.svc.cluster.local".to_string(),
        8123,
        "username".to_string(),
        "password".to_string(),
        db.to_string(),
    )
}
