use common::prelude::{Encoding, ExchangeID, ProtocolType, ServiceID, ServiceType};
use ims_data_binance_specs::ims_data_binance_config;

#[test]
fn test_ims_data_binance_config() {
    let service_config = ims_data_binance_config();

    // Test all the accessors for the ServiceConfig struct
    assert_eq!(service_config.svc_id(), &ServiceID::ImsDataBinance);
    assert_eq!(
        service_config.name(),
        format!("ims-service-{}", ExchangeID::Binance)
    );
    assert_eq!(service_config.version(), 1);
    assert_eq!(service_config.online(), false);
    assert_eq!(
        service_config.description(),
        format!(
            "IMS controls streaming data for {} exchange",
            ExchangeID::Binance
        )
    );
    assert_eq!(
        service_config.health_check_uri(),
        format!(
            "ims-data-service-{}.default.svc.cluster.local:7070/health",
            ExchangeID::Binance
        )
    );
    assert_eq!(
        service_config.base_uri(),
        format!(
            "ims-data-service-{}.default.svc.cluster.local",
            ExchangeID::Binance
        )
    );
    assert_eq!(service_config.dependencies(), &vec![ServiceID::SMDB]);
    assert_eq!(service_config.exposure(), &ServiceType::ENDPOINT);

    // Test the accessors for the Endpoint
    let endpoint = service_config.endpoint();
    assert_eq!(
        endpoint.name(),
        format!("{}-ims-data-endpoint", ExchangeID::Binance)
    );
    assert_eq!(endpoint.version(), 1);
    assert_eq!(
        endpoint.description(),
        "Control Binance exchange streaming data data via gRPC on baseUri:7070"
    );
    assert_eq!(endpoint.uri(), "/");
    assert_eq!(endpoint.port(), 7070);
    assert_eq!(endpoint.protocol(), ProtocolType::GRPC);
    assert_eq!(endpoint.encoding(), Encoding::Protobuf);

    // Test the accessors for the MetricConfig
    let metric_config = service_config.metrics();
    assert_eq!(metric_config.uri(), "metrics");
    assert_eq!(metric_config.host(), "0.0.0.0");
    assert_eq!(metric_config.port(), 8080);
}
