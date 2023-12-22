use cfg_manager::CfgManager;
use common::errors::InitError;
use common::prelude::{Endpoint, ServiceConfig, ServiceID};

pub struct ServiceManager<'l> {
    cfg_manager: &'l CfgManager<'l>,
}

impl<'l> ServiceManager<'l> {
    /// new_offline_service_manager creates an offline service manager with only DB access
    /// required to implement SMDB service registry.
    pub fn new(cfg_manager: &'l CfgManager) -> Self {
        Self { cfg_manager }
    }
}

impl<'l> ServiceManager<'l> {
    pub fn configure_svc_socket_addr(&self, svc_id: &ServiceID) -> Result<String, InitError> {
        if !self.is_service_initialized(svc_id) {
            self.init_service(svc_id)
                .expect("[ServiceManager]: Failed to initialize service");
        }

        self.cfg_manager.configure_svc_socket_addr(svc_id)
    }

    pub fn configure_metrics_socket_addr_uri(
        &self,
        svc_id: &ServiceID,
    ) -> Result<(String, String), InitError> {
        self.cfg_manager.configure_metrics_socket_addr_uri(svc_id)
    }

    /// Returns a reference to the service-specific configuration of the service.
    pub fn get_service_config(&self) -> ServiceConfig {
        self.cfg_manager.get_svc_config()
    }

    pub fn get_service_dependencies(&self) -> Vec<ServiceID> {
        self.cfg_manager.get_svc_config().dependencies().clone()
    }

    pub fn get_service_metric_host_uri_port(
        &self,
        svc_id: &ServiceID,
    ) -> Result<(String, String, u16), InitError> {
        if !self.is_service_initialized(svc_id) {
            self.init_service(svc_id)
                .expect("[ServiceManager]: Failed to initialize service");
        }

        let svc_metric = self.cfg_manager.get_svc_metric_config_by_id(svc_id);
        let metric_host = svc_metric.metric_host().to_string();
        let metric_uri = svc_metric.metric_uri().to_string();
        let metrics_port = svc_metric.metric_port();

        Ok((metric_host, metric_uri, metrics_port))
    }

    pub fn get_service_endpoint(&self, svc_id: &ServiceID) -> Endpoint {
        self.cfg_manager.get_svc_config_by_id(svc_id).endpoint()
    }

    pub fn get_service_host_port(&self, svc_id: &ServiceID) -> Result<(String, u16), InitError> {
        if !self.is_service_initialized(svc_id) {
            self.init_service(svc_id)
                .expect("[ServiceManager]: Failed to initialize service");
        }

        self.cfg_manager.get_svc_host_port(svc_id)
    }
}

impl<'l> ServiceManager<'l> {
    fn is_service_initialized(&self, dependency: &ServiceID) -> bool {
        self.cfg_manager.is_svc_env_initialized(dependency)
    }

    fn init_service(&self, svc_id: &ServiceID) -> Result<(), InitError> {
        let svc_config = self.cfg_manager.get_svc_config_by_id(svc_id).to_owned();
        let binding = svc_config.endpoint();
        let endpoint = binding.host_endpoint();
        let metrics_config = svc_config.metrics().to_owned();

        self.cfg_manager
            .init_svc_env(svc_id, endpoint, metrics_config)
    }
}
