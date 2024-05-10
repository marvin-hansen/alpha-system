use config_manager::CfgManager;
use std::env;

use common::prelude::{EnvironmentType, ServiceID};
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
use service_specs::prelude::{cmdb_service_config, dbgw_service_config, smdb_service_config};

#[test]
fn test_get_cmdb_host() {
    env::set_var("ENV", "LOCAL");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);
    assert_eq!(ctm.int_dns_server(), &None);

    let dnm = DnsManager::new(&ctm);
    let cfg_manager = CfgManager::new(ServiceID::CMDB, cmdb_service_config(), &ctm, &dnm);

    let host = cfg_manager.get_svc_host_port().unwrap();
    assert_eq!(host, ("0.0.0.0".to_string(), 7070));
}

#[test]
fn test_get_smdb_host() {
    env::set_var("ENV", "LOCAL");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);
    assert_eq!(ctm.int_dns_server(), &None);

    let dnm = DnsManager::new(&ctm);
    let cfg_manager = CfgManager::new(ServiceID::SMDB, smdb_service_config(), &ctm, &dnm);
    let host = cfg_manager.get_svc_host_port().unwrap();
    assert_eq!(host, ("0.0.0.0".to_string(), 7070));
}

#[test]
fn test_get_dbgw_host() {
    env::set_var("ENV", "LOCAL");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);
    assert_eq!(ctm.int_dns_server(), &None);

    let dnm = DnsManager::new(&ctm);
    let cfg_manager = CfgManager::new(ServiceID::DBGW, dbgw_service_config(), &ctm, &dnm);

    let host = cfg_manager.get_svc_host_port().unwrap();
    assert_eq!(host, ("0.0.0.0".to_string(), 9090));
}
