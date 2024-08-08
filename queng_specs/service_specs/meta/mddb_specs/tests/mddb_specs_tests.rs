use common_config::prelude::{ProtocolType, ServiceID};
use mddb_specs::mddb_service_config;

#[test]
fn test_mddb_service_config_accessors() {
    let service_config = mddb_service_config();

    // Test all the accessors for the ServiceConfig struct
    assert_eq!(service_config.svc_id(), &ServiceID::MDDB);
    assert_eq!(service_config.name(), "mddbv1".to_string());
    assert_eq!(service_config.version(), 1);
    assert!(!service_config.online());
    assert_eq!(
        service_config.description(),
        "MDDB gives access to meta data".to_string()
    );
    assert_eq!(
        service_config.health_check_uri(),
        "dbgwv1-service.default.svc.cluster.local:7070/health".to_string()
    );
    assert_eq!(
        service_config.base_uri(),
        "mddbv1-service.default.svc.cluster.local".to_string()
    );
    assert_eq!(service_config.dependencies(), &vec![]);

    // Test the accessors for the Endpoint
    let endpoint = service_config.service_endpoint();
    assert_eq!(endpoint.name(), "mddb Endpoint".to_string());
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.uri(), "/".to_string());
    assert_eq!(endpoint.port(), 7070);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);

    // Test the accessors for the MetricConfig
    let metric_config = service_config.metrics_endpoint();
    assert_eq!(metric_config.uri(), "metrics".to_string());
    assert_eq!(metric_config.host(), "0.0.0.0".to_string());
    assert_eq!(metric_config.port(), 8080);
}
