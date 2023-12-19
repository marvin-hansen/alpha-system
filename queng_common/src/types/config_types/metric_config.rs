use proto::binding::ProtoMetricConfig;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MetricConfig {
    metric_uri: String,
    metric_host: String,
    metric_port: u16,
}

impl MetricConfig {
    pub fn new(metric_uri: String, metric_host: String, metric_port: u16) -> Self {
        Self {
            metric_uri,
            metric_host,
            metric_port,
        }
    }
}

impl MetricConfig {
    pub fn from_proto(proto: ProtoMetricConfig) -> Result<MetricConfig, fmt::Error> {
        Ok(MetricConfig {
            metric_uri: proto.metric_uri,
            metric_host: proto.metric_host,
            metric_port: proto.metric_port as u16,
        })
    }

    pub fn to_proto(&self) -> Result<ProtoMetricConfig, fmt::Error> {
        Ok(ProtoMetricConfig {
            metric_uri: self.metric_uri.clone(),
            metric_host: self.metric_host.clone(),
            metric_port: self.metric_port as u32,
        })
    }
}

impl Default for MetricConfig {
    fn default() -> Self {
        Self {
            metric_uri: String::from("metrics"),
            metric_host: String::from("127.0.0.1"),
            metric_port: 8080,
        }
    }
}

impl MetricConfig {
    pub fn metric_uri(&self) -> &str {
        &self.metric_uri
    }
    pub fn metric_host(&self) -> &str {
        &self.metric_host
    }
    pub fn metric_port(&self) -> u16 {
        self.metric_port
    }
}

impl Display for MetricConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "metric_uri: {},  metric_host: {},  metric_port: {}",
            self.metric_uri, self.metric_host, self.metric_port
        )
    }
}
