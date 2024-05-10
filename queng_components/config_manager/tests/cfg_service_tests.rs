use config_manager::CfgManager;
use std::env;

use common::prelude::{EnvironmentType, ServiceID};
use ctx_manager::CtxManager;
use dns_manager::DnsManager;

// LOCAL and unknown environment cannot really be tested otherwise CI test runs breaks
// because the environment variable must be set in the CI environment (not in the test)
// Since you can onlu set the environment variable in the CI environment to one value,
// it was decided to test for the cluster environment as this is most critical.
// Please ensure the following is added to the CI test GH action:

//         env:
//           ENV: CLUSTER
//           DNS_SERVER: 9.9.9.9

#[test]
fn test_new() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("9.9.9.9".to_string()));

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");
}

#[test]
fn test_init_smdb_env() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("9.9.9.9".to_string()));

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");
}

#[test]
fn test_init_cmdb_env() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("9.9.9.9".to_string()));

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");
}

#[test]
fn test_init_dbgw_env() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("9.9.9.9".to_string()));

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");
}

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
