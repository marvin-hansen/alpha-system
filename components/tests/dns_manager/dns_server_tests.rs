use std::env;

use common::prelude::{DnsConfig, EnvironmentType};
use components::prelude::{CtxManager, DnsManager};

// LOCAL and unknown environment cannot really be tested otherwise CI test runs breaks
// because the environment variable must be set in the CI environment (not in the test)
// Since you can onlu set the environment variable in the CI environment to one value,
// it was decided to test for the cluster environment as this is most critical.
// Please ensure the following is added to the CI test GH action:

//         env:
//           ENV: CLUSTER
//           DNS_SERVER: 175.24.54.1

#[test]
fn test_new() {
    let key = "ENV";
    let value = "CLUSTER";
    env::set_var(key, value);

    let key = "DNS_SERVER";
    let value = "175.24.54.1";
    env::set_var(key, value);

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("175.24.54.1".to_string()));

    let dnm = DnsManager::new(DnsConfig::default(), &ctm);
    assert_eq!(dnm.internal_dns(), "175.24.54.1:53");
    assert_eq!(dnm.extern_dns(), "1.1.1.1:53");
}

#[test]
fn test_resolve_internal_dns() {
    let key = "ENV";
    let value = "CLUSTER";
    env::set_var(key, value);

    let key = "DNS_SERVER";
    let value = "175.24.54.1";
    env::set_var(key, value);

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("175.24.54.1".to_string()));

    let dnm = DnsManager::new(DnsConfig::default(), &ctm);
    assert_eq!(dnm.internal_dns(), "175.24.54.1:53");
    assert_eq!(dnm.extern_dns(), "1.1.1.1:53");

    let expected = "175.24.54.1:53".to_string();
    let actual = dnm.resolve_dns("example.com", true).unwrap();
    assert_eq!(actual, expected)
}

#[test]
fn test_display() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "175.24.54.1");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("175.24.54.1".to_string()));

    let dnm = DnsManager::new(DnsConfig::default(), &ctm);
    assert_eq!(dnm.internal_dns(), "175.24.54.1:53");
    assert_eq!(dnm.extern_dns(), "1.1.1.1:53");

    let expected = "DnsManager { internal_dns: 175.24.54.1:53, extern_dns: 1.1.1.1:53 }";
    let actual = format!("{}", dnm);
    assert_eq!(actual, expected);
}
