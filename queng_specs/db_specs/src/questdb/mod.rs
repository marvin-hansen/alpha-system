use common::prelude::QuestDBConfig;

/// Returns a DBConfig for connecting to a local QuestDB instance.
///
/// # Returns
///
/// DBConfig with:
/// - PORT set to 9009
/// - host set to "0.0.0.0"
///
/// # Remarks
///
/// Useful for connecting to a local QuestDB instance.
///
pub fn get_local_quest_db_config() -> QuestDBConfig {
    QuestDBConfig::new("0.0.0.0".into())
}

/// Returns a DBConfig for connecting to a QuestDB cluster instance.
///
/// # Returns
///
/// DBConfig with:
/// - PORT set to 9009
/// - host set to "questdb.default.svc.cluster.local"
///
/// # Remarks
///
/// Useful for connecting to a QuestDB cluster instance
/// using Kubernetes service discovery.
///
pub fn get_cluster_quest_db_config() -> QuestDBConfig {
    QuestDBConfig::new("questdb.default.svc.cluster.local".into())
}
