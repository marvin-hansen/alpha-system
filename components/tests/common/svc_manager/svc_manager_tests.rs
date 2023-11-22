use std::env;

use common::prelude::{Encoding, ProtocolType, ServiceID, ServiceType};
use components::prelude::{CfgManager, CtxManager, DnsManager, EnvManager, ServiceManager};

#[test]
fn test_new() {
    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &EnvManager::new(ctx_manager, dns_manager);
    let service_manager = ServiceManager::new(cfg_manager, svm_manager);
    let service_config = service_manager.get_service_config();
    assert_eq!(service_config.svc_id(), &ServiceID::SMDB);
}

#[test]
fn test_get_service_config() {
    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &EnvManager::new(ctx_manager, dns_manager);

    let service_manager = ServiceManager::new(cfg_manager, svm_manager);

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
fn get_service_dependencies() {
    let svc = ServiceID::DBGW;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &EnvManager::new(ctx_manager, dns_manager);

    let service_manager = ServiceManager::new(cfg_manager, svm_manager);

    let service_dependencies = service_manager.get_service_dependencies();

    assert_eq!(service_dependencies.len(), 0);
}

#[test]
fn test_get_service_endpoint() {
    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &EnvManager::new(ctx_manager, dns_manager);
    let service_manager = ServiceManager::new(cfg_manager, svm_manager);

    let endpoint = service_manager.get_service_endpoint();

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
    env::set_var("ENV", "LOCAL");

    let svc = ServiceID::SMDB;
    let ctx_manager = &CtxManager::new();
    let dns_manager = &DnsManager::new(ctx_manager);
    let cfg_manager = &CfgManager::new(svc, ctx_manager);
    let svm_manager = &EnvManager::new(ctx_manager, dns_manager);

    let service_manager = ServiceManager::new(cfg_manager, svm_manager);
    let service_config = service_manager.get_service_config();
    assert_eq!(service_config.svc_id(), &ServiceID::SMDB);

    let result = service_manager.get_service_host_port(ServiceID::SMDB);

    // Assert
    assert!(result.is_ok());

    let (host, port) = result.unwrap();
    assert_eq!(host, "127.0.0.1");
    assert_eq!(port, 7070);
}
