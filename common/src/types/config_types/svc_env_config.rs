use std::fmt::{Display, Formatter};

use crate::prelude::ServiceID;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SvcEnvConfig {
    service_id: ServiceID,
    /// The hostname address of the cluster
    cluster_host: String,
    /// The hostname address in Continuous Integration (CI) for testing
    ci_host: String,
    /// The hostname address of the local machine
    local_host: String,
    /// The port on which the service is listening
    port: String,
}

impl SvcEnvConfig {
    /// Creates a new `SvcEnvConfig` with the given parameters

    pub fn new(service_id: ServiceID, cluster_host: String, ci_host: String, local_host: String, port: String) -> Self {
        Self { service_id, cluster_host, ci_host, local_host, port }
    }
}

impl SvcEnvConfig {
    /// Returns the hostname address of the host in a cluster
    pub fn cluster_host(&self) -> &str {
        &self.cluster_host
    }
    /// Returns the hostname address of the host in Continuous Integration (CI)
    pub fn ci_host(&self) -> &str {
        &self.ci_host
    }
    /// Returns the hostname of the host on a local machine
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
        write!(
            f,
            "SvcEnvConfig {{ service_id: {:?}, cluster_host: {:?}, ci_host: {:?}, local_host: {:?}, port: {:?} }}",
            self.service_id, self.cluster_host, self.ci_host, self.local_host, self.port
        )
    }
}
