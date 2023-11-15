use std::env;

use common::prelude::{EnvironmentType, HostEndpoint, ServiceID};
use components::prelude::{CtxManager, DnsManager, SvcEnvManager};

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

    let env_manager = SvcEnvManager::new(&ctm, &dnm);
    let endpoint = HostEndpoint::new("example.com", 8080);
    assert!(env_manager.init_svc_env(&ServiceID::SMDB, endpoint).is_ok());
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

    let env_manager = SvcEnvManager::new(&ctm, &dnm);
    let endpoint = HostEndpoint::new("example.com", 8080);
    assert!(env_manager.init_svc_env(&ServiceID::SMDB, endpoint).is_ok());
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

    let env_manager = SvcEnvManager::new(&ctm, &dnm);
    let endpoint = HostEndpoint::new("example.com", 8080);
    assert!(env_manager.init_svc_env(&ServiceID::CMDB, endpoint).is_ok());
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

    let env_manager = SvcEnvManager::new(&ctm, &dnm);
    let endpoint = HostEndpoint::new("example.com", 8080);
    assert!(env_manager.init_svc_env(&ServiceID::DBGW, endpoint).is_ok());
}

#[test]
fn test_get_cmdb_host() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("9.9.9.9".to_string()));

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");

    let env_manager = SvcEnvManager::new(&ctm, &dnm);

    let endpoint = HostEndpoint::new("localhost", 7070);
    assert!(env_manager.init_svc_env(&ServiceID::CMDB, endpoint).is_ok());

    let host = env_manager.get_svc_host_port(ServiceID::CMDB).unwrap();
    assert_eq!(host, ("127.0.0.1".to_string(), 7070));
}

#[test]
fn test_get_smdb_host() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("9.9.9.9".to_string()));

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");

    let env_manager = SvcEnvManager::new(&ctm, &dnm);

    let endpoint = HostEndpoint::new("localhost", 8080);
    assert!(env_manager.init_svc_env(&ServiceID::SMDB, endpoint).is_ok());

    let host = env_manager.get_svc_host_port(ServiceID::SMDB).unwrap();
    assert_eq!(host, ("127.0.0.1".to_string(), 8080));
}

#[test]
fn test_get_memgraph_host() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("9.9.9.9".to_string()));

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");

    let env_manager = SvcEnvManager::new(&ctm, &dnm);

    let endpoint = HostEndpoint::new("localhost", 9090);
    assert!(env_manager.init_svc_env(&ServiceID::DBGW, endpoint).is_ok());

    let host = env_manager.get_svc_host_port(ServiceID::DBGW).unwrap();
    assert_eq!(host, ("127.0.0.1".to_string(), 9090));
}
