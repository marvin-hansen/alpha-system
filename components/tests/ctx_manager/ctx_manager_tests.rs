use std::env;

use common::prelude::EnvironmentType;
use components::prelude::CtxManager;

#[test]
fn test_new() {
    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::UnknownEnv);
    assert_eq!(ctm.int_dns_server(), &None);
}

#[test]
fn test_new_local() {
    let key = "ENV";
    let value = "LOCAL";
    env::set_var(key, value);

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);
    assert_eq!(ctm.int_dns_server(), &None);
}

#[test]
fn test_new_cluster() {
    let key = "ENV";
    let value = "CLUSTER";
    env::set_var(key, value);

    let key = "DNS_SERVER";
    let value = "175.24.54.1";
    env::set_var(key, value);

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("175.24.54.1".to_string()));
}

#[test]
fn test_env_type() {
    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::UnknownEnv);
}

#[test]
fn test_int_dns_server() {
    let ctm = CtxManager::new();
    assert_eq!(ctm.int_dns_server(), &None);
}

#[test]
fn test_display() {
    let ctm = CtxManager::new();
    assert_eq!(
        ctm.to_string(),
        "CtxManager { env_type: UnknownEnv, int_dns_server: None }"
    );
}
