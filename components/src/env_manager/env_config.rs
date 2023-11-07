use std::fmt::{Display, Formatter};

use common::prelude::ServiceID;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SvcEnvConfig {
    service_id: ServiceID,
    cluster_host: String,
    local_host: String,
    port: String,
}

impl SvcEnvConfig {
    pub fn new(service_id: ServiceID, cluster_host: String, local_host: String, port: String) -> Self {
        Self { service_id, cluster_host, local_host, port }
    }
}

impl SvcEnvConfig {
    pub fn cluster_host(&self) -> &str {
        &self.cluster_host
    }
    pub fn local_host(&self) -> &str {
        &self.local_host
    }
    pub fn port(&self) -> &str {
        &self.port
    }
}

impl Display for SvcEnvConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "ServiceConfig {{ service_id: {:?}, cluster_host: {:?}, local_host: {:?}, port: {:?} }}",
               self.service_id, self.cluster_host, self.local_host, self.port
        )
    }
}