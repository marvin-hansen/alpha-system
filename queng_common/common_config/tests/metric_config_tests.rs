use common_config::prelude::MetricConfig;

#[test]
fn test_new() {
    let config = MetricConfig::new("metrics".to_string(), "localhost".to_string(), 8080);

    assert_eq!(config.uri(), "metrics");
    assert_eq!(config.host(), "localhost");
    assert_eq!(config.port(), 8080);
}

#[test]
fn test_default() {
    let config = MetricConfig::default();

    assert_eq!(config.uri(), "metrics");
    assert_eq!(config.host(), "127.0.0.1");
    assert_eq!(config.port(), 8080);
}

#[test]
fn test_display() {
    let config = MetricConfig::default();

    let expected = format!(
        "metric_uri: {},  metric_host: {},  metric_port: {}",
        config.uri(),
        config.host(),
        config.port()
    );

    assert_eq!(format!("{}", config), expected);
}
