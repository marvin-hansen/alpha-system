use crate::CfgManager;
use common_errors::prelude::InitError;

impl CfgManager {
    pub async fn get_health_check_uri(&self) -> Result<String, InitError> {
        let svc = self.svc_config.to_owned();
        let health_endpoint = svc.health_endpoint();

        let health_host = self
            .get_service_host()
            .await
            .expect("[CfgManager]: Failed to get service host");
        let health_uri = health_endpoint.uri().to_string();
        let port = health_endpoint.port() as u16;
        let health_port = self
            .get_port(port)
            .expect("[CfgManager]: Failed to get port from config");

        let health_check_uri = format!("{}:{}/{}", health_host, health_port, health_uri);

        Ok(health_check_uri)
    }
}
