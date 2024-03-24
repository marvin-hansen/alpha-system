use common::prelude::MetricConfig;
use proto::binding::ProtoMetricConfig;
use std::fmt;

pub fn metric_config_from_proto(proto: ProtoMetricConfig) -> Result<MetricConfig, fmt::Error> {
    Ok(MetricConfig::new(
        proto.metric_uri,
        proto.metric_host,
        proto.metric_port as u16,
    ))
}

pub fn metric_config_to_proto(
    metric_config: &MetricConfig,
) -> Result<ProtoMetricConfig, fmt::Error> {
    Ok(ProtoMetricConfig {
        metric_uri: metric_config.metric_uri().to_string(),
        metric_host: metric_config.metric_host().to_string(),
        metric_port: metric_config.metric_port() as u32,
    })
}
