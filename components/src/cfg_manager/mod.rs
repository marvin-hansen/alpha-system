use common::prelude::{
    Endpoint, EnvironmentType, HostEndpoint, MainConfig, ServiceConfig, ServiceID,
};
use specs::prelude::{cmdb_service_config, memgraph_service_config, smdb_service_config};

use crate::prelude::CtxManager;

/// Struct that holds the configuration for a specific service.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct CfgManager {
    /// ID of the service.
    svc: ServiceID,
    /// Type of the environment (e.g., development, testing, production).
    env_type: EnvironmentType,
    /// Main configuration for the service.
    main_config: MainConfig,
    /// Service-specific configuration.
    svc_config: ServiceConfig,
}

impl CfgManager {
    /// Creates a new `ConfigManager` instance for the given service ID.
    ///
    /// # Arguments
    ///
    /// * `svc` - ID of the service.
    /// * `ctx` - Context manager.
    pub fn new(svc: ServiceID, ctx: &CtxManager) -> Self {
        let env_type = ctx.env_type();
        let svc_config = get_service_config(svc);
        let main_config = svc_config.main_config();

        Self {
            svc,
            env_type,
            main_config,
            svc_config,
        }
    }
}

impl CfgManager {
    /// Returns the ID of the service.
    pub fn svc(&self) -> ServiceID {
        self.svc
    }
    /// Returns the type of the environment.
    pub fn env_type(&self) -> EnvironmentType {
        self.env_type
    }
    /// Returns a reference to the main configuration for the service.
    pub fn main_config(&self) -> &MainConfig {
        &self.main_config
    }
    /// Returns a reference to the service-specific configuration.
    pub fn svc_config(&self) -> &ServiceConfig {
        &self.svc_config
    }

    pub fn get_svc_config(&self, svc_id: ServiceID) -> ServiceConfig {
        get_service_config(svc_id)
    }

    pub fn get_svc_endpoint(&self, svc_id: ServiceID) -> Endpoint {
        get_service_config(svc_id).endpoint().clone()
    }

    pub fn get_svc_host_endpoint(&self, svc_id: ServiceID) -> HostEndpoint {
        self.get_svc_endpoint(svc_id).clone().host_endpoint()
    }
}

// Returns the service configuration for the given service ID.
fn get_service_config(svc: ServiceID) -> ServiceConfig {
    match svc {
        ServiceID::MEMGRAPH => memgraph_service_config(),
        ServiceID::SMDB => smdb_service_config(),
        ServiceID::CMDB => cmdb_service_config(),
        ServiceID::Default => ServiceConfig::default(),
    }
}
