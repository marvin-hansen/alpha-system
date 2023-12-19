use common::prelude::MetricConfig;
use proto::binding::ProtoMetricConfig;

#[test]
fn test_new() {
    let config = MetricConfig::new("metrics".to_string(), "localhost".to_string(), 8080);

    assert_eq!(config.metric_uri(), "metrics");
    assert_eq!(config.metric_host(), "localhost");
    assert_eq!(config.metric_port(), 8080);
}

#[test]
fn test_default() {
    let config = MetricConfig::default();

    assert_eq!(config.metric_uri(), "metrics");
    assert_eq!(config.metric_host(), "127.0.0.1");
    assert_eq!(config.metric_port(), 8080);
}

#[test]
fn test_from_proto() {
    let proto = ProtoMetricConfig {
        metric_uri: "metrics".to_string(),
        metric_host: "localhost".to_string(),
        metric_port: 8080,
    };

    let config = MetricConfig::from_proto(proto).unwrap();

    assert_eq!(config.metric_uri(), "metrics");
    assert_eq!(config.metric_host(), "localhost");
    assert_eq!(config.metric_port(), 8080);
}

#[test]
fn test_to_proto() {
    let config = MetricConfig::new("metrics".to_string(), "localhost".to_string(), 8080);

    let proto = config.to_proto().unwrap();

    assert_eq!(proto.metric_uri, "metrics");
    assert_eq!(proto.metric_host, "localhost");
    assert_eq!(proto.metric_port, 8080);
}

#[test]
fn test_display() {
    let config = MetricConfig::default();

    let expected = format!(
        "metric_uri: {},  metric_host: {},  metric_port: {}",
        config.metric_uri(),
        config.metric_host(),
        config.metric_port()
    );

    assert_eq!(format!("{}", config), expected);
}
