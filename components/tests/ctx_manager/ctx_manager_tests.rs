use std::env;

use common::prelude::EnvironmentType;
use components::prelude::CtxManager;

#[test]
fn test_new() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "175.24.54.1");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("175.24.54.1".to_string()));
}

#[test]
fn test_env_type() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "175.24.54.1");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
}

#[test]
fn test_int_dns_server() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "175.24.54.1");

    let ctm = CtxManager::new();
    assert_eq!(ctm.int_dns_server(), &Some("175.24.54.1".to_string()));
}

#[test]
fn test_display() {
    let ctm = CtxManager::new();
    assert_eq!(
        ctm.to_string(),
        "CtxManager { env_type: UnknownEnv, int_dns_server: None }"
    );
}
