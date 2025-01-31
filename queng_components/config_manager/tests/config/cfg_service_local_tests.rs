/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use config_manager::CfgManager;
use std::env;

use cmdb_specs::cmdb_service_config;
use common_config::ServiceID;
use dbgw_specs::dbgw_service_config;
use smdb_specs::smdb_service_config;

#[tokio::test]
async fn test_get_cmdb_host() {
    // Environment access only happens in single-threaded code.
    unsafe { env::set_var("ENV", "LOCAL") };

    let service_id = ServiceID::CMDB;
    let cfg_manager = CfgManager::with_debug(service_id, cmdb_service_config()).await;

    let actual_host_port = cfg_manager.get_svc_host_port().await.unwrap();

    let expected_local_host = "0.0.0.0".to_string();
    let expected_local_port = 7070 + service_id.as_u16();
    assert_eq!(actual_host_port, (expected_local_host, expected_local_port));
}

#[tokio::test]
async fn test_get_smdb_host() {
    // Environment access only happens in single-threaded code.
    unsafe { env::set_var("ENV", "LOCAL") };

    let service_id = ServiceID::SMDB;
    let cfg_manager = CfgManager::with_debug(service_id, smdb_service_config()).await;

    let actual_host_port = cfg_manager.get_svc_host_port().await.unwrap();
    let expected_local_host = "0.0.0.0".to_string();
    let expected_local_port = 7070 + service_id.as_u16();
    assert_eq!(actual_host_port, (expected_local_host, expected_local_port));
}

#[tokio::test]
async fn test_get_dbgw_host() {
    // Environment access only happens in single-threaded code.
    unsafe { env::set_var("ENV", "LOCAL") };

    let service_id = ServiceID::DBGW;
    let cfg_manager = CfgManager::with_debug(service_id, dbgw_service_config()).await;

    let actual_host_port = cfg_manager.get_svc_host_port().await.unwrap();
    let expected_local_host = "0.0.0.0".to_string();
    let expected_local_port = 9090 + service_id.as_u16();
    assert_eq!(actual_host_port, (expected_local_host, expected_local_port));
}
