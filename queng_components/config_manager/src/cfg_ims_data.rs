use crate::fields::{DEFAULT_HOST, DEFAULT_IMS_PORT};
use crate::CfgManager;
use common_env::EnvironmentType;
use common_errors::InitError;
use common_exchange::ExchangeID;

impl CfgManager {
    pub fn get_ims_data_svc_health_uri(
        &self,
        exchange_id: ExchangeID,
    ) -> Result<String, InitError> {
        let uri = format!(
            "http://{}/health",
            self.get_ims_data_svc_socket_addr(exchange_id)?
        );
        Ok(uri)
    }

    pub fn get_ims_data_svc_port(&self, exchange_id: ExchangeID) -> Result<u16, InitError> {
        self.get_ims_port(&exchange_id)
    }
    pub fn get_ims_data_svc_socket_addr(
        &self,
        exchange_id: ExchangeID,
    ) -> Result<String, InitError> {
        // Set host to default (0.0.0.0) to listen on all interfaces
        let host = DEFAULT_HOST;

        // Adjust the port relative to the environment.
        let port = self
            .get_ims_port(&exchange_id)
            .expect("Failed to get port from config");

        // Merge the host and port into a socket address i.e. 0.0.0.0:7070
        let socket_addr = format!("{host}:{port}");

        Ok(socket_addr)
    }

    // Adjust the port relative to the environment.
    // For local and CI environments, the port is shifted by the exchange id
    // to prevent ports from clashing.
    fn get_ims_port(&self, exchange_id: &ExchangeID) -> Result<u16, InitError> {
        match self.env_type {
            EnvironmentType::LOCAL => Ok(DEFAULT_IMS_PORT + exchange_id.as_u16()),
            EnvironmentType::CLUSTER => Ok(DEFAULT_IMS_PORT),
            EnvironmentType::CI => Ok(DEFAULT_IMS_PORT + exchange_id.as_u16()),
            EnvironmentType::UNKNOWN => Err(InitError::new("Unknown Environment".to_string())),
        }
    }
}
