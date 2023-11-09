use common::prelude::{
    EnvironmentType, MainConfig, ServiceConfig, ServiceID,
};
use specs::prelude::{cmdb_service_config, memgraph_service_config, smdb_service_config};

use crate::prelude::CtxManager;

/// Struct that holds the configuration for a specific service.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CfgManager<'l> {
    /// ID of the service.
    svc: ServiceID,
    /// Type of the environment (e.g., development, testing, production).
    env_type: EnvironmentType,
    // Bind  the lifeline
    id: &'l str,
}

impl<'l> CfgManager<'l> {
    /// Creates a new `ConfigManager` instance for the given service ID.
    ///
    /// # Arguments
    ///
    /// * `svc` - ID of the service.
    /// * `ctx` - Context manager.
    pub fn new(svc: ServiceID, ctx: &CtxManager) -> Self {
        let env_type = ctx.env_type();
        Self {
            svc,
            env_type,
            id: "CfgManager",
        }
    }
}

impl<'l> CfgManager<'l> {
    fn service_config(&self, svc: ServiceID) -> ServiceConfig<'l> {
        match svc {
            ServiceID::MEMGRAPH => memgraph_service_config(),
            ServiceID::SMDB => smdb_service_config(),
            ServiceID::CMDB => cmdb_service_config(),
            ServiceID::Default => ServiceConfig::default(),
        }
    }
}

impl<'l> CfgManager<'l> {
    /// Returns the ID of the service.
    pub fn svc(&self) -> ServiceID {
        self.svc
    }
    /// Returns the type of the environment.
    pub fn env_type(&self) -> EnvironmentType {
        self.env_type
    }
    /// Returns a reference to the main configuration for the service.
    pub fn main_config(&self) -> MainConfig {
        self.service_config(self.svc).clone().main_config()
    }
    /// Returns a reference to the service-specific configuration.
    pub fn svc_config(&self) -> ServiceConfig {
        self.service_config(self.svc)
    }
    pub fn get_svc_config(&self, svc_id: ServiceID) -> ServiceConfig {
        self.service_config(svc_id)
    }
}

