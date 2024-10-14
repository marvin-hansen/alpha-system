use crate::CfgManager;
use common_config::prelude::ServiceID;
use common_errors::prelude::InitError;

impl CfgManager {
    pub async fn get_health_check_url(&self, service_id: &ServiceID) -> Result<String, InitError> {
        self.dbg_print("get_health_check_url");

        let svc = self.svc_config.to_owned();
        let health_endpoint = svc.health_endpoint();

        let health_host = self
            .get_service_host(&self.svc_env_config)
            .await
            .expect("[CfgManager]: Failed to get service host");

        let health_uri = health_endpoint.uri().to_string();
        let port = health_endpoint.port() as u16;
        let health_port = self
            .get_port(port, service_id)
            .expect("[CfgManager]: Failed to get port from config");

        let health_check_url = format!("http://{}:{}/{}", health_host, health_port, health_uri);

        self.dbg_print(&health_check_url);

        Ok(health_check_url)
    }
}
