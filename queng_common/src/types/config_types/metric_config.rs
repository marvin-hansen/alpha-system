use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MetricConfig {
    uri: String,
    host: String,
    port: u16,
}

impl MetricConfig {
    pub fn new(uri: String, host: String, port: u16) -> Self {
        Self { uri, host, port }
    }
}

impl Default for MetricConfig {
    fn default() -> Self {
        Self {
            uri: String::from("metrics"),
            host: String::from("127.0.0.1"),
            port: 8080,
        }
    }
}

impl MetricConfig {
    pub fn uri(&self) -> &str {
        &self.uri
    }
    pub fn host(&self) -> &str {
        &self.host
    }
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Display for MetricConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "metric_uri: {},  metric_host: {},  metric_port: {}",
            self.uri, self.host, self.port
        )
    }
}
