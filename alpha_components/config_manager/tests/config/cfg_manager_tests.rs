/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_config::ServiceID;
use common_env::EnvironmentType;

use config_manager::CfgManager;
use smdb_specs::smdb_service_config;
use std::env;

#[tokio::test]
async fn new_config_manager_default() {
    // Environment access only happens in single-threaded code.
    unsafe { env::set_var("ENV", "CLUSTER") };
    // Environment access only happens in single-threaded code.
    unsafe { env::set_var("DNS_SERVER", "9.9.9.9") };
    // On a K8s cluster, PG_USER, PG_PASSWORD and PG_DATABASE usually are set as cluster secrets
    // Environment access only happens in single-threaded code.
    unsafe { env::set_var("PG_USER", "postgres") };
    // Environment access only happens in single-threaded code.
    unsafe { env::set_var("PG_PASSWORD", "password") };
    // Environment access only happens in single-threaded code.
    unsafe { env::set_var("PG_DATABASE", "database") };

    let config_manager = CfgManager::new(ServiceID::SMDB, smdb_service_config()).await;

    assert_eq!(config_manager.get_svc_id(), ServiceID::SMDB);
    assert_eq!(config_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.get_svc_config(), smdb_service_config());
}
