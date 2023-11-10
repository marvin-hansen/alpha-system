use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::prelude::{Endpoint, MainConfig, ServiceID, ServiceType};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ServiceConfig<'l> {
    /// Unique Service ID.
    id: ServiceID,
    /// Service name.
    name: &'l str,
    /// Service version.
    version: u8,
    /// Whether the service is online.
    online: bool,
    /// Service description.
    description: &'l str,
    /// Health check URI.
    health_check_uri: &'l str,
    /// Base URI.
    base_uri: &'l str,
    /// Service dependencies.
    dependencies: Vec<ServiceID>,
    /// Service exposure type.
    exposure: ServiceType,
    /// Service endpoint.
    endpoint: Endpoint<'l>,
}

impl<'l> ServiceConfig<'l> {
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
        id: ServiceID,
        name: &'l str,
        version: u8,
        online: bool,
        description: &'l str,
        health_check_uri: &'l str,
        base_uri: &'l str,
        dependencies: Vec<ServiceID>,
        exposure: ServiceType,
        endpoint: Endpoint<'l>,
    ) -> Self {
        Self {
            id,
            name,
            version,
            online,
            description,
            health_check_uri,
            base_uri,
            dependencies,
            exposure,
            endpoint,
        }
    }
}

impl<'l> ServiceConfig<'l> {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        // https://github.com/serde-rs/json
        let json = serde_json::to_string(&self)
            .expect("Failed to serialize ServiceConfig to JSON");

        Ok(json)
    }

    pub fn to_memgraph(&self) -> String {
        // "{id: 1, name:'Memgraph', description: 'Fastest graph DB in the world!', createdAt: Date()})",

        let s = format!("{{id: {}, name: '{}', version: {}, online: {}, description: '{}', \
        health_check_uri: '{}', base_uri: '{}', dependencies: {}, exposure: '{}', endpoint: {}}}",
                        self.id.to_uint(), self.name, self.version, self.online, self.description,
                        self.health_check_uri, self.base_uri, format_dependencies(&self.dependencies),
                        self.exposure, self.endpoint.to_memgraph());

        s
    }
}

fn format_dependencies(dependencies: &Vec<ServiceID>) -> String {
    let mut s = String::new();
    s.push_str("[");
    for dependency in dependencies {
        let value = dependency.to_string();
        s.push_str(&format!("'{}'", value));
    }
    s.push_str("]");
    s
}

impl<'l> ServiceConfig<'l> {
    /// Returns the main configuration for the service.
    pub fn main_config(&self) -> MainConfig {
        MainConfig::new(
            *self.id(),
            String::from(self.name()),
            self.endpoint().port(),
            self.endpoint().protocol(),
        )
    }
}

impl<'l> ServiceConfig<'l> {
    /// Returns the service ID.
    pub fn id(&self) -> &ServiceID {
        &self.id
    }
    /// Returns the service name.
    pub fn name(&self) -> &str {
        self.name
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
        self.description
    }
    /// Returns the health check URI.
    pub fn health_check_uri(&self) -> &str {
        self.health_check_uri
    }
    /// Returns the base URI.
    pub fn base_uri(&self) -> &str {
        self.base_uri
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
}

impl<'l> Display for ServiceConfig<'l> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "ServiceConfig {{ id: {}, name: {}, version: {}, online: {}, description: {}, health_check_uri: {}, base_uri: {}, dependencies: {:?}, exposure: {}, endpoint: {} }}",
               self.id, self.name, self.version, self.online, self.description, self.health_check_uri, self.base_uri, self.dependencies, self.exposure, self.endpoint
        )
    }
}
