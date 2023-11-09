use common::errors::InitError;
use common::prelude::{MainConfig, ServiceConfig, ServiceID};

use crate::env_manager::SvcEnvManager;
use crate::prelude::CfgManager;

const ONLINE: bool = true;
const OFFLINE: bool = false;

pub struct ServiceManager<'l> {
    cfg_manager: &'l CfgManager<'l>,
    svm_manager: &'l mut SvcEnvManager<'l>,
    online: bool,
}

impl<'l> ServiceManager<'l> {
    /// new_online_service_manager creates a normal service manager for usage in all services that register with the SMDB
    pub fn new_online_service_manager(
        cfg_manager: &'l CfgManager,
        svm_manager: &'l mut SvcEnvManager<'l>,
    ) -> Self {
        Self {
            cfg_manager,
            svm_manager,
            online: ONLINE,
        }
    }

    /// new_offline_service_manager creates an offline service manager with only DB access
    /// required to implement SMDB service registry.
    pub fn new_offline_service_manager(
        cfg_manager: &'l CfgManager,
        svm_manager: &'l mut SvcEnvManager<'l>,
    ) -> Self {
        Self {
            cfg_manager,
            svm_manager,
            online: OFFLINE,
        }
    }
}

impl<'l> ServiceManager<'l> {
    /// Returns true if the service is online and can reach the SMDB registry
    pub fn is_online(&self) -> bool {
        self.online
    }

    /// Returns a reference to the main configuration of the service.
    pub fn get_service_main_config(&self) -> MainConfig {
        self.cfg_manager.main_config()
    }

    /// Returns a reference to the service-specific configuration of the service.
    pub fn get_service_config(&self) -> ServiceConfig {
        self.cfg_manager.svc_config()
    }
}

impl<'l> ServiceManager<'l> {
    pub fn init_service_dependencies(
        &mut self,
        dependencies: Vec<ServiceID>,
    ) -> Result<(), InitError> {
        for svc_id in dependencies {
            self.init_service(svc_id)?
        }

        Ok(())
    }

    fn init_service(&mut self, svc_id: ServiceID) -> Result<(), InitError> {
        let svc_config = self.cfg_manager.get_svc_config(svc_id).to_owned();
        let binding = svc_config.endpoint();
        let endpoint = binding.host_endpoint();
        self.svm_manager.init_svc_env(svc_id, endpoint)
    }
}
