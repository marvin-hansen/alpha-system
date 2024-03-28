use std::env;

use common::prelude::EnvironmentType;
use ctx_manager::CtxManager;

// Please ensure the following is added to the CI test GH action:
//         env:
//           ENV: LOCAL

// Emulate ENV variable
fn setup() {
    env::set_var("ENV", "LOCAL");
}

#[test]
fn test_new() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);
    assert_eq!(ctm.int_dns_server(), &None);
}

#[test]
fn test_env_type() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::LOCAL);
}

#[test]
fn test_int_dns_server() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(ctm.int_dns_server(), &None);
}

#[test]
fn test_display() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(
        ctm.to_string(),
        "CtxManager { env_type: LOCAL, int_dns_server: None }"
    );
}
