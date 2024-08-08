use common_config::prelude::{ProtocolType, ServiceID};
use ims_data_vex_specs::vex_service_config;

#[test]
fn test_vex_service_config_accessors() {
    let service_config = vex_service_config();

    // Test all the accessors for the ServiceConfig struct
    assert_eq!(service_config.svc_id(), &ServiceID::VEX);
    assert_eq!(service_config.name(), "vexv1");
    assert_eq!(service_config.version(), 1);
    assert!(!service_config.online());
    assert_eq!(service_config.description(), "vex: Virtual Exchange");
    assert_eq!(
        service_config.health_check_uri(),
        "vex-service.default.svc.cluster.local:9999/health"
    );
    assert_eq!(
        service_config.base_uri(),
        "vex-service.default.svc.cluster.local"
    );
    assert_eq!(service_config.dependencies(), &vec![ServiceID::SMDB]);

    // Test the accessors for the Endpoint
    let endpoint = service_config.service_endpoint();
    assert_eq!(endpoint.name(), "vex-ims-data-endpoint");
    assert_eq!(endpoint.version(), 1);
    assert_eq!(endpoint.uri(), "/");
    assert_eq!(endpoint.port(), 7070);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);

    // Test the accessors for the MetricConfig
    let metric_config = service_config.metrics_endpoint();
    assert_eq!(metric_config.uri(), "metrics");
    assert_eq!(metric_config.host(), "0.0.0.0");
    assert_eq!(metric_config.port(), 8080);
}
