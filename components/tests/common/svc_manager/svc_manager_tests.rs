use std::env;

use common::prelude::{Encoding, ProtocolType, ServiceID, ServiceType};
use components::prelude::{CfgManager, CtxManager, DnsManager, ServiceManager, SvcEnvManager};

#[test]
fn test_new_offline_service_manager() {
    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &SvcEnvManager::new(ctx_manager, dns_manager);
    let service_manager = ServiceManager::new_offline_service_manager(cfg_manager, svm_manager);
    let service_config = service_manager.get_service_config();
    assert_eq!(service_config.svc_id(), &ServiceID::SMDB);
}

#[test]
fn test_get_service_config() {
    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &SvcEnvManager::new(ctx_manager, dns_manager);

    let service_manager = ServiceManager::new_offline_service_manager(cfg_manager, svm_manager);

    let service_config = service_manager.get_service_config();
    assert_eq!(service_config.svc_id(), &ServiceID::SMDB);
    assert_eq!(service_config.name(), "smdbv1");
    assert_eq!(service_config.version(), 1);
    assert_eq!(
        service_config.description(),
        "SMDB Service Management Database"
    );
    assert_eq!(
        service_config.health_check_uri(),
        "smdb-service.default.svc.cluster.local:5050/health"
    );
    assert_eq!(
        service_config.base_uri(),
        "smdb-service.default.svc.cluster.local"
    );
    assert_eq!(
        service_config.dependencies().len(),
        vec![ServiceID::DBGW].len()
    );
    assert_eq!(service_config.exposure(), &ServiceType::ENDPOINT);

    let endpoint = service_config.endpoint();
    assert_eq!(endpoint.name(), "service-registry");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(
        endpoint.description(),
        "Access to the SMDB service registry via gRPC on baseUri:7070"
    );
    assert_eq!(endpoint.uri(), "/");
    assert_eq!(endpoint.port(), 7070);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
    assert_eq!(endpoint.encoding(), Encoding::Protobuf);
}

#[test]
fn test_get_service_host_port() {
    // Make this conditional to run in CI
    env::set_var("ENV", "LOCAL");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &SvcEnvManager::new(ctx_manager, dns_manager);

    let service_manager = ServiceManager::new_offline_service_manager(cfg_manager, svm_manager);
    let service_config = service_manager.get_service_config();
    assert_eq!(service_config.svc_id(), &ServiceID::SMDB);

    // We can't really test this, because the CI can't resolve the DNS server of the cluster host.
    // The root cause is that the CI can only have one ENV variable and it's alerady set to CLUSTER.
    // assert!(service_manager
    //     .get_service_host_port(ServiceID::CMDB)
    //     .is_err());

    // assert_eq!(
    //     service_manager.get_service_host_port(ServiceID::CMDB).unwrap(),
    //     String::from("127.0.0.1:7070")
    // );

    // assert_eq!(
    //     service_manager.get_service_host_port(ServiceID::SMDB).unwrap(),
    //     String::from("127.0.0.1:5050")
    // );

    // assert_eq!(
    //     service_manager.get_service_host_port(ServiceID::MEMGRAPH).unwrap(),
    //     String::from("127.0.0.1:7687")
    // );
}
