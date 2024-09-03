use common_config::prelude::ServiceID;
use common_env::prelude::EnvironmentType;
use config_manager::CfgManager;
use smdb_specs::smdb_service_config;
use std::env;

#[tokio::test]
async fn test_env_type() {
    env::set_var("ENV", "CI");

    let config_manager = CfgManager::new(ServiceID::SMDB, smdb_service_config()).await;

    assert_eq!(config_manager.env_type(), EnvironmentType::CI);
}
