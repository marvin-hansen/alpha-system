use common::prelude::{
    DBConfig, EnvironmentType, FileConfig, FileConfigType, MessageClientConfig, ServiceConfig,
    ServiceID,
};
use ctx_manager::CtxManager;
use db_specs::prelude::{db_config_ci, db_config_cluster, db_config_local};
use file_specs::prelude::{get_all_file_config_types, get_all_file_configs};
use service_specs::prelude::{
    cmdb_service_config, dbgw_service_config, qdgw_service_config, smdb_service_config,
    vex_service_config,
};
use std::collections::HashMap;

/// Struct that holds the configuration for a specific service.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CfgManager {
    /// ID of the service.
    svc: ServiceID,
    /// Type of the environment (e.g., development, testing, production).
    env_type: EnvironmentType,
    /// File configurations for data files.
    file_configs: HashMap<FileConfigType, FileConfig>,
    ///
    file_config_types: Vec<FileConfigType>,
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
        let file_configs = get_all_file_configs();
        let file_config_types = get_all_file_config_types();

        Self {
            svc,
            env_type,
            file_configs,
            file_config_types,
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
    pub fn get_file_config(&self, file_config_type: &FileConfigType) -> Option<&FileConfig> {
        self.file_configs.get(file_config_type)
    }
    pub fn get_all_file_config_types(&self) -> Vec<FileConfigType> {
        self.file_config_types.clone()
    }
    pub fn get_message_client_config(&self) -> MessageClientConfig {
        MessageClientConfig::from_svc_id(self.svc)
    }
}

impl CfgManager {
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
            ServiceID::VEX => vex_service_config(),
            ServiceID::Default => ServiceConfig::default(),
        }
    }
}
