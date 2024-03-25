use common::prelude::MetricConfig;
use proto_bindings::proto::ProtoMetricConfig;
use proto_utils::metric_config_proto_utils::{metric_config_from_proto, metric_config_to_proto};

#[test]
fn test_from_proto() {
    let proto = ProtoMetricConfig {
        metric_uri: "metrics".to_string(),
        metric_host: "localhost".to_string(),
        metric_port: 8080,
    };

    let config = metric_config_from_proto(proto).unwrap();

    assert_eq!(config.metric_uri(), "metrics");
    assert_eq!(config.metric_host(), "localhost");
    assert_eq!(config.metric_port(), 8080);
}

#[test]
fn test_to_proto() {
    let config = MetricConfig::new("metrics".to_string(), "localhost".to_string(), 8080);

    let proto = metric_config_to_proto(&config).unwrap();

    assert_eq!(proto.metric_uri, "metrics");
    assert_eq!(proto.metric_host, "localhost");
    assert_eq!(proto.metric_port, 8080);
}
