use std::env;

use common_env::prelude::EnvironmentType;
use ctx_manager::CtxManager;
use dns_manager::DnsManager;

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
async fn test_internal_dns() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);

    let dnm = DnsManager::new(&ctm).await;
    assert_eq!(dnm.internal_dns_server(), "9.9.9.9:53");
}

// We cannot test the internal DNS resolution since
// this would require a custom DNS server with custom DNS records.
// However, this is build for Kubernetes and usually k8s DNS works reliably.

#[tokio::test]
async fn test_external_dns() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);

    let dnm = DnsManager::new(&ctm).await;
    assert_eq!(dnm.internal_dns_server(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns_server(), "1.1.1.1:53");
}

#[tokio::test]
async fn test_resolve_external_dns() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);

    let dnm = DnsManager::new(&ctm).await;
    assert_eq!(dnm.internal_dns_server(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns_server(), "1.1.1.1:53");

    let host = "harvard.edu";
    let res = dnm.resolve_dns(host, false).await;
    assert!(res.is_ok());

    let host = "mit.edu";
    let res = dnm.resolve_dns(host, false).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_display() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new().await;
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);

    let dnm = DnsManager::new(&ctm).await;
    assert_eq!(dnm.to_string(), "DnsManager: \n env_type: CLUSTER \n internal_dns_server: 9.9.9.9:53 \n external_dns_server 1.1.1.1:53");
}
