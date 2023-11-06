use common::prelude::EnvironmentType;
use components::prelude::CtxManager;

#[test]
fn test_new() {
    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::UnknownEnv);
    assert_eq!(ctm.int_dns_server(), &None);
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
    assert_eq!(ctm.to_string(), "CtxManager { env_type: UnknownEnv, int_dns_server: None }");
}