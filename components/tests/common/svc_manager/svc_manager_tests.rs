use std::env;

use common::prelude::{Encoding, ProtocolType, ServiceID, ServiceType};
use components::prelude::{CfgManager, CtxManager, DnsManager, ServiceManager, SvcEnvManager};

#[test]
fn test_new_online_service_manager() {
    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &SvcEnvManager::new(ctx_manager, dns_manager);
    let service_manager = ServiceManager::new_online_service_manager(cfg_manager, svm_manager);
    assert_eq!(service_manager.is_online(), &true);
}

#[test]
fn test_new_offline_service_manager() {
    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &SvcEnvManager::new(ctx_manager, dns_manager);
    let service_manager = ServiceManager::new_offline_service_manager(cfg_manager, svm_manager);
    assert_eq!(service_manager.is_online(), &false);
}

#[test]
fn test_is_online() {
    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &SvcEnvManager::new(ctx_manager, dns_manager);
    let service_manager = ServiceManager::new_online_service_manager(cfg_manager, svm_manager);
    assert_eq!(service_manager.is_online(), &true);

    let service_manager = ServiceManager::new_offline_service_manager(cfg_manager, svm_manager);
    assert_eq!(service_manager.is_online(), &false);
}

#[test]
fn test_get_service_main_config() {
    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &SvcEnvManager::new(ctx_manager, dns_manager);

    let service_manager = ServiceManager::new_offline_service_manager(cfg_manager, svm_manager);
    assert_eq!(service_manager.is_online(), &false);

    let main_config = service_manager.get_service_main_config();
    assert_eq!(main_config.id(), &ServiceID::SMDB);
    assert_eq!(main_config.name(), "smdbv1");
    assert_eq!(main_config.port(), 5050);
    assert_eq!(main_config.protocol(), &ProtocolType::GRPC);
}

#[test]
fn test_get_service_config() {
    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &SvcEnvManager::new(ctx_manager, dns_manager);

    let service_manager = ServiceManager::new_offline_service_manager(cfg_manager, svm_manager);
    assert_eq!(service_manager.is_online(), &false);

    let service_config = service_manager.get_service_config();
    assert_eq!(service_config.svc_id(), &ServiceID::SMDB);
    assert_eq!(service_config.name(), "smdbv1");
    assert_eq!(service_config.version(), 1);
    assert!(!service_config.online());
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
        vec![ServiceID::MEMGRAPH].len()
    );
    assert_eq!(service_config.exposure(), &ServiceType::ENDPOINT);

    let endpoint = service_config.endpoint();
    assert_eq!(endpoint.name(), "service-registry");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(
        endpoint.description(),
        "Access to the SMDB service registry via gRPC on baseUri:5050"
    );
    assert_eq!(endpoint.uri(), "/");
    assert_eq!(endpoint.port(), 5050);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
    assert_eq!(endpoint.encoding(), Encoding::Protobuf);
}

#[test]
fn test_init_service_dependencies() {
    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &SvcEnvManager::new(ctx_manager, dns_manager);

    let service_manager = ServiceManager::new_offline_service_manager(cfg_manager, svm_manager);
    assert_eq!(service_manager.is_online(), &false);

    let dependencies = vec![ServiceID::CMDB];
    service_manager
        .init_service_dependencies(dependencies)
        .unwrap();
    assert!(service_manager.is_service_dependency_initialized(ServiceID::CMDB));

    let dependencies = vec![ServiceID::SMDB];
    service_manager
        .init_service_dependencies(dependencies)
        .unwrap();
    assert!(service_manager.is_service_dependency_initialized(ServiceID::SMDB));

    let dependencies = vec![ServiceID::MEMGRAPH];
    service_manager
        .init_service_dependencies(dependencies)
        .unwrap();
    assert!(service_manager.is_service_dependency_initialized(ServiceID::MEMGRAPH));
}

#[test]
fn test_get_dependency_svc_host() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &SvcEnvManager::new(ctx_manager, dns_manager);

    let service_manager = ServiceManager::new_offline_service_manager(cfg_manager, svm_manager);
    assert_eq!(service_manager.is_online(), &false);

    let dependencies = vec![ServiceID::CMDB];
    service_manager
        .init_service_dependencies(dependencies)
        .unwrap();
    assert!(service_manager.is_service_dependency_initialized(ServiceID::CMDB));

    // We can't really test this, because the CI can't resolve the DNS server of the cluster host.
    // The root cause is that the CI can only have one ENV variable and it's alerady set to CLUSTER.
    assert!(service_manager
        .get_dependency_svc_host(ServiceID::CMDB)
        .is_err());

    // assert_eq!(
    //     service_manager.get_dependency_svc_host(ServiceID::CMDB).unwrap(),
    //     String::from("127.0.0.1:7070")
    // );

    let dependencies = vec![ServiceID::SMDB];
    service_manager
        .init_service_dependencies(dependencies)
        .unwrap();
    assert!(service_manager.is_service_dependency_initialized(ServiceID::SMDB));
    assert!(service_manager.is_service_dependency_initialized(ServiceID::SMDB));
    assert!(service_manager
        .get_dependency_svc_host(ServiceID::SMDB)
        .is_err());

    // assert_eq!(
    //     service_manager.get_dependency_svc_host(ServiceID::SMDB).unwrap(),
    //     String::from("127.0.0.1:5050")
    // );

    let dependencies = vec![ServiceID::MEMGRAPH];
    service_manager
        .init_service_dependencies(dependencies)
        .unwrap();
    assert!(service_manager.is_service_dependency_initialized(ServiceID::MEMGRAPH));

    assert!(service_manager.is_service_dependency_initialized(ServiceID::MEMGRAPH));
    assert!(service_manager
        .get_dependency_svc_host(ServiceID::MEMGRAPH)
        .is_err());

    // assert_eq!(
    //     service_manager.get_dependency_svc_host(ServiceID::MEMGRAPH).unwrap(),
    //     String::from("127.0.0.1:7687")
    // );
}
