use common_config::prelude::ServiceID;
use common_env::prelude::EnvironmentType;

use config_manager::CfgManager;
use smdb_specs::smdb_service_config;
use std::env;

#[tokio::test]
async fn new_config_manager_default() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let config_manager = CfgManager::new(ServiceID::SMDB, smdb_service_config()).await;

    assert_eq!(config_manager.get_svc_id(), ServiceID::SMDB);
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.get_svc_config(), smdb_service_config());
}
