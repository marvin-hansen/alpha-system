use common::prelude::{DBConfig, EnvironmentType, ServiceConfig, ServiceID};
use ctx_manager::CtxManager;
use specs::prelude::{
    cmdb_service_config, db_config_ci, db_config_cluster, db_config_local, dbgw_service_config,
    qdgw_service_config, smdb_service_config,
};

/// Struct that holds the configuration for a specific service.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CfgManager<'l> {
    /// ID of the service.
    svc: ServiceID,
    /// Type of the environment (e.g., development, testing, production).
    env_type: EnvironmentType,
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
    /// Returns the ID of the service.
    pub fn svc(&self) -> ServiceID {
        self.svc
    }
    /// Returns the type of the environment.
    pub fn env_type(&self) -> EnvironmentType {
        self.env_type
    }
    /// Returns a reference to the service-specific configuration.
    pub fn svc_config(&self) -> ServiceConfig {
        self.service_config(&self.svc)
    }
    pub fn get_svc_config(&self, svc_id: &ServiceID) -> ServiceConfig {
        self.service_config(svc_id)
    }

    pub fn get_db_config(&self) -> DBConfig {
        self.db_config()
    }
}

impl<'l> CfgManager<'l> {
    fn db_config(&self) -> DBConfig {
        match self.env_type {
            EnvironmentType::LOCAL => db_config_local(),
            EnvironmentType::CI => db_config_ci(),
            EnvironmentType::CLUSTER => db_config_cluster(),
            _ => panic!("Invalid environment type"),
        }
    }
    fn service_config(&self, svc: &ServiceID) -> ServiceConfig {
        match svc {
            ServiceID::SMDB => smdb_service_config(),
            ServiceID::CMDB => cmdb_service_config(),
            ServiceID::DBGW => dbgw_service_config(),
            ServiceID::QDGW => qdgw_service_config(),
            ServiceID::Default => ServiceConfig::default(),
        }
    }
}
