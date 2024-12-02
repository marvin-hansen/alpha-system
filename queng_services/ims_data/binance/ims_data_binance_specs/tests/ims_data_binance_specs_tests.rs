use common_config::{ProtocolType, ServiceID};
use ims_data_binance_specs::ims_data_binance_config;

#[test]
fn test_ims_data_binance_config() {
    let service_config = ims_data_binance_config();

    // Test all the accessors for the ServiceConfig struct
    assert_eq!(service_config.svc_id(), &ServiceID::ImsDataBinance);
    assert_eq!(service_config.name(), "ims-service-Binance");
    assert_eq!(service_config.version(), 1);
    assert!(!service_config.online());
    assert_eq!(
        service_config.description(),
        "IMS controls streaming data for Binance exchange",
    );
    assert_eq!(
        service_config.health_check_uri(),
        "ims-data-service-Binance.default.svc.cluster.local:7070/health",
    );
    assert_eq!(
        service_config.cluster_uri(),
        "ims-data-service-Binance.default.svc.cluster.local"
    );
    assert_eq!(service_config.dependencies(), &vec![ServiceID::SMDB]);

    // Test the accessors for the Endpoint
    let endpoint = service_config.service_endpoint();
    assert_eq!(endpoint.name(), "Binance-ims-data-endpoint");
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
