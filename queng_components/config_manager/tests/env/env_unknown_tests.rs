/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_config::ServiceID;
use common_env::EnvironmentType;
use config_manager::CfgManager;
use smdb_specs::smdb_service_config;
use std::env;

#[tokio::test]
async fn test_env_type() {
    // Environment access only happens in single-threaded code.
    unsafe { env::set_var("ENV", "UNKNOWN") };

    let config_manager = CfgManager::new(ServiceID::SMDB, smdb_service_config()).await;

    assert_eq!(config_manager.env_type(), EnvironmentType::UNKNOWN);
}
