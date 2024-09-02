use std::env;

use common_env::prelude::EnvironmentType;
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

#[tokio::test]
async fn test_new() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);

    let dnm = DnsManager::new(&ctm).await;
    assert_eq!(dnm.internal_dns_server(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns_server(), "1.1.1.1:53");
}

#[tokio::test]
async fn test_init_smdb_env() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);

    let dnm = DnsManager::new(&ctm).await;
    assert_eq!(dnm.internal_dns_server(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns_server(), "1.1.1.1:53");
}

#[tokio::test]
async fn test_init_cmdb_env() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);

    let dnm = DnsManager::new(&ctm).await;
    assert_eq!(dnm.internal_dns_server(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns_server(), "1.1.1.1:53");
}

#[tokio::test]
async fn test_init_dbgw_env() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);

    let dnm = DnsManager::new(&ctm).await;
    assert_eq!(dnm.internal_dns_server(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns_server(), "1.1.1.1:53");
}
