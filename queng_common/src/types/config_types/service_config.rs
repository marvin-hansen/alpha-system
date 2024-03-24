use crate::prelude::{Endpoint, MetricConfig, ServiceID, ServiceType};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct ServiceConfig {
    // DB ID
    id: Option<Thing>,
    /// Unique Service ID.
    svc_id: ServiceID,
    /// Service name.
    name: String,
    /// Service version.
    version: u8,
    /// Whether the service is online.
    online: bool,
    /// Service description.
    description: String,
    /// Health check URI.
    health_check_uri: String,
    /// Base URI.
    base_uri: String,
    /// Service dependencies.
    dependencies: Vec<ServiceID>,
    /// Service exposure type.
    exposure: ServiceType,
    /// Service endpoint.
    endpoint: Endpoint,
    /// Service metrics
    metrics: MetricConfig,
}

impl ServiceConfig {
    /// Creates a new `ServiceConfig` instance.
    ///
    /// # Arguments
    ///
    /// * `id` - Service ID.
    /// * `name` - Service name.
    /// * `version` - Service version.
    /// * `online` - Whether the service is online.
    /// * `description` - Service description.
    /// * `health_check_uri` - Health check URI.
    /// * `base_uri` - Base URI.
    /// * `dependencies` - Service dependencies.
    /// * `exposure` - Service exposure type.
    /// * `endpoint` - Service endpoint.
    // https://rust-lang.github.io/rust-clippy/master/index.html#/too_many_arguments
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        svc_id: ServiceID,
        name: String,
        version: u8,
        online: bool,
        description: String,
        health_check_uri: String,
        base_uri: String,
        dependencies: Vec<ServiceID>,
        exposure: ServiceType,
        endpoint: Endpoint,
        metrics: MetricConfig,
    ) -> Self {
        Self {
            id: None,
            svc_id,
            name,
            version,
            online,
            description,
            health_check_uri,
            base_uri,
            dependencies,
            exposure,
            endpoint,
            metrics,
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
    pub fn version(&self) -> u8 {
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
    pub fn base_uri(&self) -> &str {
        &self.base_uri
    }
    /// Returns the service dependencies.
    pub fn dependencies(&self) -> &Vec<ServiceID> {
        &self.dependencies
    }
    /// Returns the service exposure type.
    pub fn exposure(&self) -> &ServiceType {
        &self.exposure
    }
    /// Returns the service endpoint.
    pub fn endpoint(&self) -> Endpoint {
        self.endpoint.to_owned()
    }
    pub fn metrics(&self) -> &MetricConfig {
        &self.metrics
    }
}

impl Display for ServiceConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "ServiceConfig {{ svc_id: {}, name: {}, version: {}, online: {}, description: {}, health_check_uri: {}, base_uri: {}, dependencies: {:?}, exposure: {}, endpoint: {} metrics: {} }}",
               self.svc_id, self.name, self.version, self.online, self.description, self.health_check_uri, self.base_uri, self.dependencies, self.exposure, self.endpoint, self.metrics
        )
    }
}
