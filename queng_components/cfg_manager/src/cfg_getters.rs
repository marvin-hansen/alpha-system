use crate::CfgManager;
use common::prelude::{
    DBConfig, EnvironmentType, FileConfig, FileConfigType, MessageClientConfig, ServiceConfig,
    ServiceID,
};
use db_specs::prelude::{db_config_ci, db_config_cluster, db_config_local};
use service_specs::prelude::{
    cmdb_service_config, dbgw_service_config, qdgw_service_config, smdb_service_config,
    vex_service_config,
};

impl<'l> CfgManager<'l> {
    /// Returns the ID of the service.
    pub fn get_svc_id(&self) -> ServiceID {
        self.svc
    }
    /// Returns the type of the environment.
    pub fn get_env_type(&self) -> EnvironmentType {
        self.env_type
    }
    /// Returns a reference to the service-specific configuration.
    pub fn get_svc_config(&self) -> ServiceConfig {
        self.service_config(&self.svc)
    }
    pub fn get_svc_config_by_id(&self, svc_id: &ServiceID) -> ServiceConfig {
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
            ServiceID::VEX => vex_service_config(),
            ServiceID::Default => ServiceConfig::default(),
        }
    }
}
