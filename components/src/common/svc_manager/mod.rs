use common::errors::InitError;
use common::prelude::{Endpoint, ServiceConfig, ServiceID};

use crate::prelude::CfgManager;
use crate::prelude::SvcEnvManager;

const ONLINE: &bool = &true;
const OFFLINE: &bool = &false;

pub struct ServiceManager<'l> {
    cfg_manager: &'l CfgManager<'l>,
    svm_manager: &'l SvcEnvManager<'l>,
    online: &'l bool,
}

impl<'l> ServiceManager<'l> {
    // TODO implement online service manager that connects to SMDB service

    /// new_online_service_manager creates a normal service manager for usage in all services that register with the SMDB
    pub fn new_online_service_manager(
        cfg_manager: &'l CfgManager,
        svm_manager: &'l SvcEnvManager<'l>,
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
        svm_manager: &'l SvcEnvManager<'l>,
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
    pub fn is_online(&self) -> &bool {
        self.online
    }

    /// Returns a reference to the service-specific configuration of the service.
    pub fn get_service_config(&self) -> ServiceConfig {
        self.cfg_manager.svc_config()
    }

    pub fn get_service_endpoint(&self) -> Endpoint {
        self.cfg_manager.svc_config().endpoint()
    }

    pub fn get_service_host_port(&self, svc_id: ServiceID) -> Result<(String, u16), InitError> {
        if !self.is_service_initialized(&svc_id) {
            self.init_service(&svc_id)
                .expect("Failed to initialize service");
        }

        self.svm_manager.get_svc_host_port(svc_id)
    }
}

impl<'l> ServiceManager<'l> {
    fn is_service_initialized(&self, dependency: &ServiceID) -> bool {
        self.svm_manager.is_svc_env_initialized(dependency)
    }

    fn init_service(&self, svc_id: &ServiceID) -> Result<(), InitError> {
        let svc_config = self.cfg_manager.get_svc_config(svc_id).to_owned();
        let binding = svc_config.endpoint();
        let endpoint = binding.host_endpoint();
        self.svm_manager.init_svc_env(svc_id, endpoint)
    }
}
