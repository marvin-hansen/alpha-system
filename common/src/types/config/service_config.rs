use std::fmt::{Display, Formatter};

use crate::types::config::endpoint::Endpoint;
use crate::types::config::main_config::MainConfig;
use crate::types::config::service_id::ServiceID;
use crate::types::config::service_type::ServiceType;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ServiceConfig {
    id: ServiceID,
    name: String,
    version: u8,
    online: bool,
    description: String,
    health_check_uri: String,
    base_uri: String,
    dependencies: Vec<ServiceID>,
    exposure: ServiceType,
    endpoint: Endpoint,
}

impl ServiceConfig {
    // https://rust-lang.github.io/rust-clippy/master/index.html#/too_many_arguments
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ServiceID,
        name: String,
        version: u8,
        online: bool,
        description: String,
        health_check_uri: String,
        base_uri: String,
        dependencies: Vec<ServiceID>,
        exposure: ServiceType,
        endpoint: Endpoint,
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

impl ServiceConfig {
    pub fn get_main_config(&self) -> MainConfig {
        MainConfig::new(
            *self.id(),
            String::from(self.name()),
            self.endpoint().port(),
            *self.endpoint().protocol(),
        )
    }
}

impl ServiceConfig {
    pub fn id(&self) -> &ServiceID {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn version(&self) -> u8 {
        self.version
    }
    pub fn online(&self) -> bool {
        self.online
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn health_check_uri(&self) -> &str {
        &self.health_check_uri
    }
    pub fn base_uri(&self) -> &str {
        &self.base_uri
    }
    pub fn dependencies(&self) -> &Vec<ServiceID> {
        &self.dependencies
    }
    pub fn exposure(&self) -> &ServiceType {
        &self.exposure
    }
    pub fn endpoint(&self) -> &Endpoint {
        &self.endpoint
    }
}

impl Display for ServiceConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "ServiceConfig {{ id: {}, name: {}, version: {}, online: {}, description: {}, health_check_uri: {}, base_uri: {}, dependencies: {:?}, exposure: {}, endpoint: {} }}",
               self.id, self.name, self.version, self.online, self.description, self.health_check_uri, self.base_uri, self.dependencies, self.exposure, self.endpoint
        )
    }
}
