use cmdb_specs::cmdb_service_config;
use common_config::prelude::ServiceID::{CMDB, DBGW, SMDB};
use common_config::prelude::{ServiceID, SvcEnvConfig};
use common_env::prelude::EnvironmentType;
use common_errors::prelude::InitError;
use dbgw_specs::dbgw_service_config;
use smdb_specs::smdb_service_config;

use crate::utils::get_svc_env_config;
use crate::{CfgManager, DEFAULT_HOST};

impl CfgManager {
    /// Returns the host and port of the service.
    ///
    /// # Returns
    ///
    /// A tuple containing the host and port of the service as a string and u16, respectively. Returns an error if the
    /// host and port cannot be obtained.
    pub async fn get_smdb_host_port(&self) -> Result<(String, u16), InitError> {
        // Get the configuration of the service
        let svc_config = smdb_service_config();

        // Construct contextual service environment configuration
        let svc_env_config = get_svc_env_config(SMDB, &svc_config);

        // Get the host and port of the service
        self.get_host(&svc_env_config).await
    }

    /// Returns the host and port of the CMDB service.
    ///
    /// # Returns
    ///
    /// A tuple containing the host and port of the CMDB service as a string and u16, respectively. Returns an error if the
    /// host and port cannot be obtained.
    ///
    pub async fn get_cmdb_host_port(&self) -> Result<(String, u16), InitError> {
        // Get the configuration of the service
        let svc_config = cmdb_service_config();

        // Construct contextual service environment configuration
        let svc_env_config = get_svc_env_config(CMDB, &svc_config);

        // Get the host and port of the service
        self.get_host(&svc_env_config).await
    }

    /// Returns the host and port of the DBGW service.
    ///
    /// # Returns
    ///
    /// A tuple containing the host and port of the DBGW service as a string and u16, respectively. Returns an error if the
    /// host and port cannot be obtained.
    pub async fn get_dbgw_host_port(&self) -> Result<(String, u16), InitError> {
        // Get the configuration of the service
        let svc_config = dbgw_service_config();

        // Construct contextual service environment configuration
        let svc_env_config = get_svc_env_config(DBGW, &svc_config);

        // Get the host and port of the service
        self.get_host(&svc_env_config).await
    }

    /// Returns the host and port of the service.
    ///
    /// # Returns
    ///
    /// A tuple containing the host and port of the service as a string and u16, respectively. Returns an error if the
    /// host and port cannot be obtained.
    pub async fn get_service_host_port(&self) -> Result<(String, u16), InitError> {
        // Get the configuration of the service
        let svc_config = &self.svc_env_config;
        // Get the host and port of the service
        self.get_host(svc_config).await
    }

    /// Returns a vector of `ServiceID`s representing the dependencies of the service.
    ///
    /// # Returns
    ///
    /// A vector of `ServiceID`s representing the dependencies of the service.
    pub fn get_service_dependencies(&self) -> Vec<ServiceID> {
        self.get_svc_config().dependencies().clone()
    }

    /// returns the socket address to run the service in any context.
    pub async fn get_svc_socket_addr(&self) -> Result<String, InitError> {
        // Get the configuration of the service
        let svc_config = self.svc_env_config.to_owned();
        // Get the host and port of the service
        let (_, port) = self
            .get_host(&svc_config)
            .await
            .expect("Failed to get host and port");
        // Set host to default (0.0.0.0) to listen on all interfaces
        let host = DEFAULT_HOST;
        // Merge the host and port into a socket address i.e. 0.0.0.0:7070
        let socket_addr = format!("{}:{}", host, port);

        Ok(socket_addr)
    }

    /// Returns the metric socket address and uri to run the service in any
    pub fn get_metrics_socket_addr_uri(&self) -> Result<(String, String), InitError> {
        let (metrics_host, metrics_uri, metrics_port) = self
            .get_svc_metric_host_uri_port()
            .expect("Failed to get metric host, uri, and port");

        // Merge the host and port into a socket address i.e. 0.0.0.0:8080
        let socket_addr = format!("{}:{}", metrics_host, metrics_port);

        Ok((socket_addr, metrics_uri))
    }

    fn get_svc_metric_host_uri_port(&self) -> Result<(String, String, u32), InitError> {
        let svc = self.svc_env_config.to_owned();
        let metric_host = svc.metrics_host().to_string();
        let metrics_uri = svc.metrics_uri().to_string();
        let port = *svc.metrics_port() as u16;

        let metrics_port = self
            .get_port(&svc, port)
            .expect("[EnvManager]: Failed to get port from config");

        Ok((metric_host, metrics_uri, metrics_port as u32))
    }

    // Returns the hostname and port of the service based on the environment type.
    // If the environment type is local, it returns the hostname of the service running locally.
    // If the environment type is cluster, it returns the hostname of the service running in the cluster.
    // If the environment type is unknown, it returns an error.
    pub(crate) async fn get_host(
        &self,
        svc_env_config: &SvcEnvConfig,
    ) -> Result<(String, u16), InitError> {
        //
        let svc_port: u16 = svc_env_config
            .service_port()
            .parse()
            .expect("[EnvManager]: Failed to parse port from config");

        let port = self
            .get_port(svc_env_config, svc_port)
            .expect("[EnvManager]: Failed to get port from config");

        let host = match self.env_type {
            EnvironmentType::LOCAL => svc_env_config.local_host().to_string(),

            EnvironmentType::CI => svc_env_config.ci_host().to_string(),

            EnvironmentType::CLUSTER => {
                let cluster_host = self
                    .resolve_dns(svc_env_config.cluster_host(), true)
                    .await
                    .expect("[EnvManager]: Failed to resolve DNS");

                cluster_host.to_string()
            }

            EnvironmentType::UNKNOWN => {
                return Err(InitError("[EnvManager]: Unknown Environment".to_string()));
            }
        };

        Ok((host, port))
    }

    /// Returns the port of the service based on the environment type.
    ///
    /// If the environment type is local, it returns the port of the service running locally.
    /// If the environment type is cluster, it returns the port of the service running in the cluster.
    /// If the environment type is unknown, it returns an error.
    ///
    pub(crate) fn get_port(
        &self,
        svc_env_config: &SvcEnvConfig,
        svc_port: u16,
    ) -> Result<u16, InitError> {
        let port = match self.env_type {
            EnvironmentType::UNKNOWN => svc_port,
            EnvironmentType::LOCAL => svc_port + svc_env_config.service_id().as_u16(),
            EnvironmentType::CLUSTER => svc_port,
            EnvironmentType::CI => svc_port + svc_env_config.service_id().as_u16(),
        };

        Ok(port)
    }
}
