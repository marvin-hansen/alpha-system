use std::fmt::{Display, Formatter};

use crate::prelude::{Endpoint, MetricConfig, ServiceID};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ServiceConfig {
    /// Service ID.
    svc_id: ServiceID,
    /// Service name.
    name: String,
    /// Service version.
    version: u32,
    /// Whether the service is online.
    online: bool,
    /// Service description.
    description: String,
    /// Health check URI.
    health_check_uri: String,
    /// Base URI.
    cluster_uri: String,
    /// Service dependencies.
    dependencies: Vec<ServiceID>,
    /// Service endpoint.
    endpoints: Vec<Endpoint>,
}

impl ServiceConfig {
    /// Creates a new `ServiceConfig` instance.
    ///
    /// # Arguments
    ///
    /// * `svc_id` - Service ID.
    /// * `name` - Service name.
    /// * `version` - Service version.
    /// * `online` - Whether the service is online.
    /// * `description` - Service description.
    /// * `health_check_uri` - Health check URI.
    /// * `cluster_uri` - CLuster URI.
    /// * `dependencies` - Service dependencies.
    /// * `endpoints` - Service endpoint.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        svc_id: ServiceID,
        name: String,
        version: u32,
        online: bool,
        description: String,
        health_check_uri: String,
        cluster_uri: String,
        dependencies: Vec<ServiceID>,
        endpoints: Vec<Endpoint>,
    ) -> Self {
        if endpoints.is_empty() {
            panic!("endpoints cannot be empty");
        }

        if endpoints.len() < 2 {
            panic!("endpoints cannot be less than 2. Just must specify at least a service endpoint, a metrics endpoint, and a health endpoint");
        }

        Self {
            svc_id,
            name,
            version,
            online,
            description,
            health_check_uri,
            cluster_uri,
            dependencies,
            endpoints,
        }
    }
}

impl ServiceConfig {
    /// Returns the service ID.
    pub fn svc_id(&self) -> &ServiceID {
        &self.svc_id
    }
    /// Returns the service name.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the service version.
    pub fn version(&self) -> u32 {
        self.version
    }
    /// Returns whether the service is online.
    pub fn online(&self) -> bool {
        self.online
    }
    /// Returns the service description.
    pub fn description(&self) -> &str {
        &self.description
    }
    /// Returns the health check URI.
    pub fn health_check_uri(&self) -> &str {
        &self.health_check_uri
    }
    /// Returns the base URI.
    pub fn cluster_uri(&self) -> &str {
        &self.cluster_uri
    }
    /// Returns the service dependencies.
    pub fn dependencies(&self) -> &Vec<ServiceID> {
        &self.dependencies
    }
    /// Returns all endpoints of the service
    pub fn endpoints(&self) -> &Vec<Endpoint> {
        &self.endpoints
    }
    /// Returns only the service endpoint.
    pub fn service_endpoint(&self) -> Endpoint {
        self.endpoints.first().unwrap().to_owned()
    }
    /// Returns only the metrics endpoint.
    pub fn metrics_endpoint(&self) -> MetricConfig {
        let endpoint = &self.endpoints.get(1).unwrap().to_owned();
        MetricConfig::from_endpoint(endpoint)
    }
    /// Returns an option to the health endpoint.
    pub fn health_endpoint(&self) -> Endpoint {
        self.endpoints.get(2).unwrap().to_owned()
    }
}

impl Display for ServiceConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "ServiceConfig {{ svc_id: {}, name: {}, version: {}, online: {}, description: {}, health_check_uri: {}, cluster_uri: {}, dependencies: {:?}, endpoint: {} metrics: {} health: {:?} }}",
               self.svc_id, self.name, self.version, self.online, self.description, self.health_check_uri, self.cluster_uri, self.dependencies, self.service_endpoint(), self.metrics_endpoint(), self.health_endpoint(),
        )
    }
}
