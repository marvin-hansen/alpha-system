use config_manager::CfgManager;
use std::env;

use common::prelude::{EnvironmentType, ServiceID};
use ctx_manager::CtxManager;
use dns_manager::DnsManager;

#[test]
fn test_get_cmdb_host() {
    env::set_var("ENV", "LOCAL");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);
    assert_eq!(ctm.int_dns_server(), &None);

    let dnm = DnsManager::new(&ctm);
    let cfg_manager = CfgManager::new(ServiceID::CMDB, &ctm, &dnm);

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
    let cfg_manager = CfgManager::new(ServiceID::SMDB, &ctm, &dnm);
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
    let cfg_manager = CfgManager::new(ServiceID::DBGW, &ctm, &dnm);

    let host = cfg_manager.get_svc_host_port().unwrap();
    assert_eq!(host, ("0.0.0.0".to_string(), 9090));
}
