use std::env;

use common::prelude::EnvironmentType;
use ctx_manager::CtxManager;

// LOCAL and unknown environment cannot really be tested otherwise CI test runs breaks
// because the environment variable must be set in the CI environment (not in the test)
// Since you can onlu set the environment variable in the CI environment to one value,
// it was decided to test for the cluster environment as this is most critical.
// Please ensure the following is added to the CI test GH action:

//         env:
//           ENV: CLUSTER
//           DNS_SERVER: 175.24.54.1

fn setup() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "175.24.54.1");
}

#[test]
fn test_new() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("175.24.54.1".to_string()));
}

#[test]
fn test_env_type() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
}

#[test]
fn test_int_dns_server() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(ctm.int_dns_server(), &Some("175.24.54.1".to_string()));
}

#[test]
fn test_display() {
    setup();

    let ctm = CtxManager::new();
    assert_eq!(
        ctm.to_string(),
        "CtxManager { env_type: CLUSTER, int_dns_server: Some(\"175.24.54.1\") }"
    );
}
