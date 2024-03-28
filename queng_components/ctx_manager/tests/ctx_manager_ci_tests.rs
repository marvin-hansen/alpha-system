use std::env;

use common::prelude::EnvironmentType;
use ctx_manager::CtxManager;

// Please ensure the following is added to the CI test GH action:
//         env:
//           ENV: CI
//           DNS_SERVER: 175.11.54.1

fn setup() {
    env::set_var("ENV", "CI");
    env::set_var("DNS_SERVER", "175.11.54.1");
}

#[test]
fn test_new() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CI);
    assert_eq!(ctm.int_dns_server(), &Some("175.11.54.1".to_string()));
}

#[test]
fn test_env_type() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CI);
    assert_eq!(ctm.int_dns_server(), &Some("175.11.54.1".to_string()));
}

#[test]
fn test_int_dns_server() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(ctm.int_dns_server(), &Some("175.11.54.1".to_string()));
}

#[test]
fn test_display() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(
        ctm.to_string(),
        "CtxManager { env_type: CI, int_dns_server: Some(\"175.11.54.1\") }"
    );
}
