use common_config::prelude::MetricConfig;
use proto_bindings::proto::ProtoMetricConfig;
use std::fmt;

/// Converts a `ProtoMetricConfig` into a `MetricConfig`.
///
/// This function takes a `ProtoMetricConfig` and converts it into a `MetricConfig` struct.
/// It extracts the necessary fields from the `ProtoMetricConfig` and constructs a new `MetricConfig` with them.
///
/// # Errors
///
/// If the conversion of `metric_port` to `u16` fails, an `fmt::Error` is returned.
///
pub fn metric_config_from_proto(proto: ProtoMetricConfig) -> Result<MetricConfig, fmt::Error> {
    Ok(MetricConfig::new(
        proto.metric_uri,
        proto.metric_host,
        proto.metric_port as u16,
    ))
}

/// Converts a `MetricConfig` into a `ProtoMetricConfig`.
///
/// This function takes a `MetricConfig` reference and converts it into a `ProtoMetricConfig` struct.
/// It extracts the necessary fields from the `MetricConfig` and constructs a new `ProtoMetricConfig` with them.
///
/// # Errors
///
/// If the conversion of `port` to `u32` fails, an `fmt::Error` is returned.
///
pub fn metric_config_to_proto(
    metric_config: &MetricConfig,
) -> Result<ProtoMetricConfig, fmt::Error> {
    Ok(ProtoMetricConfig {
        metric_uri: metric_config.uri().to_string(),
        metric_host: metric_config.host().to_string(),
        metric_port: metric_config.port() as u32,
    })
}
