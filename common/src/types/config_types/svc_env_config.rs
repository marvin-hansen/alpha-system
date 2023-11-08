use std::fmt::{Display, Formatter};

use crate::prelude::ServiceID;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SvcEnvConfig {
    service_id: ServiceID,
    /// The hostname or IP address of the cluster
    cluster_host: String,
    /// The hostname or IP address of the local machine
    local_host: String,
    /// The port on which the service is listening
    port: String,
}

impl SvcEnvConfig {
    /// Creates a new `SvcEnvConfig` with the given parameters
    pub fn new(
        service_id: ServiceID,
        cluster_host: String,
        local_host: String,
        port: String,
    ) -> Self {
        Self {
            service_id,
            cluster_host,
            local_host,
            port,
        }
    }
}

impl SvcEnvConfig {
    /// Returns the hostname or IP address of the cluster
    pub fn cluster_host(&self) -> &str {
        &self.cluster_host
    }
    /// Returns the hostname or IP address of the local machine
    pub fn local_host(&self) -> &str {
        &self.local_host
    }
    /// Returns the port on which the service is listening
    pub fn port(&self) -> &str {
        &self.port
    }
    /// Returns the service ID
    pub fn service_id(&self) -> ServiceID {
        self.service_id
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
