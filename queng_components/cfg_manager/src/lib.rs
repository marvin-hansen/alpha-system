use common::prelude::{EnvironmentType, FileConfig, FileConfigType, ServiceID, SvcEnvConfig};
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
use file_specs::prelude::{get_all_file_config_types, get_all_file_configs};
use std::cell::RefCell;
use std::collections::HashMap;

mod cfg_getters;
mod cfg_services;

// https://stackoverflow.com/questions/20778771/what-is-the-difference-between-0-0-0-0-127-0-0-1-and-localhost
const DEFAULT_HOST: &str = "0.0.0.0";

/// Struct that holds the configuration for a specific service.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CfgManager<'l> {
    ctx_manager: &'l CtxManager,
    dns_manager: &'l DnsManager,
    // ID of the service.
    svc: ServiceID,
    // Type of the environment (e.g., development, testing, production).
    env_type: EnvironmentType,
    // File configurations for data files.
    file_configs: HashMap<FileConfigType, FileConfig>,
    //
    file_config_types: Vec<FileConfigType>,
    //
    cmdb_env: RefCell<Option<SvcEnvConfig>>,
    smdb_env: RefCell<Option<SvcEnvConfig>>,
    dbgw_env: RefCell<Option<SvcEnvConfig>>,
    qdgw_env: RefCell<Option<SvcEnvConfig>>,
    vex_env: RefCell<Option<SvcEnvConfig>>,
}

impl<'l> CfgManager<'l> {
    pub fn new(svc: ServiceID, ctx_manager: &'l CtxManager, dns_manager: &'l DnsManager) -> Self {
        let env_type = ctx_manager.env_type();
        let file_configs = get_all_file_configs();
        let file_config_types = get_all_file_config_types();

        Self {
            ctx_manager,
            dns_manager,
            svc,
            env_type,
            file_configs,
            file_config_types,
            cmdb_env: RefCell::new(None),
            smdb_env: RefCell::new(None),
            dbgw_env: RefCell::new(None),
            qdgw_env: RefCell::new(None),
            vex_env: RefCell::new(None),
        }
    }
}
