use crate::{CfgManager, DEFAULT_HOST};
use common::prelude::{EnvironmentType, InitError, ServiceID, SvcEnvConfig};

impl<'l> CfgManager<'l> {
    pub fn get_service_host_port(&self) -> Result<(String, u16), InitError> {
        // Get the configuration of the service
        let svc_config = &self.svc_env_config;
        // Get the host and port of the service
        self.get_host(svc_config)
    }

    pub fn get_service_dependencies(&self) -> Vec<ServiceID> {
        self.get_svc_config().dependencies().clone()
    }

    /// returns the socket address to run the service in any context.
    pub fn get_svc_socket_addr(&self) -> Result<String, InitError> {
        // Get the configuration of the service
        let svc_config = self.svc_env_config.to_owned();
        // Get the host and port of the service
        let (_, port) = self
            .get_host(&svc_config)
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
}

impl<'l> CfgManager<'l> {
    fn get_svc_metric_host_uri_port(&self) -> Result<(String, String, u16), InitError> {
        let svc = self.svc_env_config.to_owned();
        let metric_host = svc.metrics_host().to_string();
        let metrics_uri = svc.metrics_uri().to_string();
        let metrics_port = *svc.metrics_port();

        Ok((metric_host, metrics_uri, metrics_port))
    }

    // Returns the hostname and port of the service based on the environment type.
    // If the environment type is local, it returns the hostname of the service running locally.
    // If the environment type is cluster, it returns the hostname of the service running in the cluster.
    // If the environment type is unknown, it returns an error.
    pub(crate) fn get_host(
        &self,
        svc_env_config: &SvcEnvConfig,
    ) -> Result<(String, u16), InitError> {
        //
        let port: u16 = svc_env_config
            .service_port()
            .parse()
            .expect("[EnvManager]: Failed to parse port from config");

        let host = match self.ctx_manager.env_type() {
            EnvironmentType::LOCAL => svc_env_config.local_host().to_string(),

            EnvironmentType::CI => svc_env_config.ci_host().to_string(),

            EnvironmentType::CLUSTER => {
                let cluster_host = self
                    .dns_manager
                    .resolve_dns(svc_env_config.cluster_host(), true)
                    .expect("[EnvManager]: Failed to resolve DNS");

                cluster_host.to_string()
            }

            EnvironmentType::UnknownEnv => {
                return Err(InitError("[EnvManager]: Unknown Environment".to_string()));
            }
        };

        Ok((host, port))
    }
}
