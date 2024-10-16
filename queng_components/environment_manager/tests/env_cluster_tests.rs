use common_env::prelude::EnvironmentType;
use environment_manager::EnvironmentManager;
use std::env;

#[test]
fn test_cluster_env_type() {
    env::set_var("ENV", "CLUSTER");

    let config_manager = EnvironmentManager::new();

    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
}
