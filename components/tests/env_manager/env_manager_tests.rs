use std::env;

use common::prelude::{EnvironmentType, HostEndpoint};
use components::env_manager::EnvironmentManager;
use components::prelude::{CtxManager, DnsManager};

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

    let env_manager = EnvironmentManager::new(&ctm, &dnm);
    // These return errors because the corresponding init function has not been called.
    assert!(env_manager.get_cmdb_host().is_err());
    assert!(env_manager.get_smdb_host().is_err());
    assert!(env_manager.get_memgraph_host().is_err());
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

    let mut env_manager = EnvironmentManager::new(&ctm, &dnm);
    let endpoint = HostEndpoint::new("example.com".to_string(), 8080);
    assert!(env_manager.init_smdb_env(endpoint).is_ok());
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

    let mut env_manager = EnvironmentManager::new(&ctm, &dnm);
    let endpoint = HostEndpoint::new("example.com".to_string(), 8080);
    assert!(env_manager.init_cmdb_env(endpoint).is_ok());
}

#[test]
fn test_init_memgraph_env() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("9.9.9.9".to_string()));

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");

    let mut env_manager = EnvironmentManager::new(&ctm, &dnm);
    let endpoint = HostEndpoint::new("example.com".to_string(), 8080);
    assert!(env_manager.init_memgraph_env(endpoint).is_ok());
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

    let mut env_manager = EnvironmentManager::new(&ctm, &dnm);

    let endpoint = HostEndpoint::new("localhost".to_string(), 7070);
    assert!(env_manager.init_cmdb_env(endpoint).is_ok());

    let host = env_manager.get_cmdb_host().unwrap();
    assert_eq!(host, "127.0.0.1:7070");
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

    let mut env_manager = EnvironmentManager::new(&ctm, &dnm);

    let endpoint = HostEndpoint::new("localhost".to_string(), 8080);
    assert!(env_manager.init_smdb_env(endpoint).is_ok());

    let host = env_manager.get_smdb_host().unwrap();
    assert_eq!(host, "127.0.0.1:8080");
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

    let mut env_manager = EnvironmentManager::new(&ctm, &dnm);

    let endpoint = HostEndpoint::new("localhost".to_string(), 9090);
    assert!(env_manager.init_memgraph_env(endpoint).is_ok());

    let host = env_manager.get_memgraph_host().unwrap();
    assert_eq!(host, "127.0.0.1:9090");
}
