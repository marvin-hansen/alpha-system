use common::prelude::{EnvironmentType, ServiceID};
use config_manager::CfgManager;
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
use service_specs::prelude::{
    cmdb_service_config, dbgw_service_config, qdgw_service_config, smdb_service_config,
};
use std::env;

#[test]
fn new_config_manager_smdb() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("9.9.9.9".to_string()));

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");

    let config_manager = CfgManager::new(ServiceID::SMDB, &ctm, &dnm);

    assert_eq!(config_manager.get_svc_id(), ServiceID::SMDB);
    assert_eq!(config_manager.get_env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.get_svc_config(), smdb_service_config());
}

#[test]
fn new_config_manager_cmdb() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("9.9.9.9".to_string()));

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");

    let config_manager = CfgManager::new(ServiceID::CMDB, &ctm, &dnm);

    assert_eq!(config_manager.get_svc_id(), ServiceID::CMDB);
    assert_eq!(config_manager.get_env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.get_svc_config(), cmdb_service_config());
}

#[test]
fn new_config_manager_dbgw() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("9.9.9.9".to_string()));

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");

    let config_manager = CfgManager::new(ServiceID::DBGW, &ctm, &dnm);

    assert_eq!(config_manager.get_svc_id(), ServiceID::DBGW);
    assert_eq!(config_manager.get_env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.get_svc_config(), dbgw_service_config());
}

#[test]
fn new_config_manager_qdgw() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("9.9.9.9".to_string()));

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");

    let config_manager = CfgManager::new(ServiceID::QDGW, &ctm, &dnm);

    assert_eq!(config_manager.get_svc_id(), ServiceID::QDGW);
    assert_eq!(config_manager.get_env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.get_svc_config(), qdgw_service_config());
}

#[test]
fn test_get_db_config_local() {
    env::set_var("ENV", "LOCAL");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);
    assert_eq!(ctm.int_dns_server(), &None);

    let dnm = DnsManager::new(&ctm);
    assert_eq!(ctm.int_dns_server(), &None);
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");
}

// #[test]
// fn test_get_db_config_ci() {
//     // CI uses the cluster environment variable to emulate the cluster environment
//     // therefore no dedicated CI test
//     env::set_var("ENV", "CLUSTER");
//
//     let ctx = CtxManager::new();
//     let cfg_manager = CfgManager::new(ServiceID::DBGW, &ctx);
//     let db_config = cfg_manager.get_db_config();
//     // CI
//     assert_eq!(db_config, db_config_cluster());
// }

//
// #[test]
// fn test_get_db_config_cluster() {
//     env::set_var("ENV", "CLUSTER");
//
//     let ctx = CtxManager::new();
//     let cfg_manager = CfgManager::new(ServiceID::DBGW, &ctx);
//     let db_config = cfg_manager.get_db_config();
//     assert_eq!(db_config, db_config_cluster());
// }
