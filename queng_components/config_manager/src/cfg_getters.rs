use common_config::{ServiceConfig, ServiceID};
use common_errors::InitError;

use crate::CfgManager;

impl CfgManager {
    pub const fn get_svc_id(&self) -> ServiceID {
        self.svc
    }

    pub fn get_svc_config(&self) -> ServiceConfig {
        self.svc_config.clone()
    }

    pub async fn get_svc_host_port(&self) -> Result<(String, u16), InitError> {
        // Get the configuration of the service
        let svc_config = &self.svc_env_config;
        // Get the host and port of the service
        self.get_host(svc_config).await
    }
}
