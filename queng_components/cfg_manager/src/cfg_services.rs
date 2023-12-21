use crate::{CfgManager, DEFAULT_HOST};
use common::errors::InitError;
use common::prelude::{EnvironmentType, HostEndpoint, MetricConfig, ServiceID, SvcEnvConfig};

impl<'l> CfgManager<'l> {
    /// Initializes the service environment based on the given service ID and host endpoint.
    ///
    /// # Arguments
    /// * `svc_id` - The service ID of the service to be initialized.
    /// * `endpoint` - The host endpoint of the service.
    ///
    /// # Returns
    /// `Result<(), InitError>` containing a
    /// * `InitError` in case of an error
    /// * `Ok(())` if the service environment was successfully initialized.
    pub fn init_svc_env(
        &self,
        svc_id: &ServiceID,
        endpoint: HostEndpoint,
        metrics_config: MetricConfig,
    ) -> Result<(), InitError> {
        match svc_id {
            ServiceID::CMDB => {
                let cmdb_env = self.get_svc_env_config(ServiceID::CMDB, endpoint, metrics_config);
                *self.cmdb_env.borrow_mut() = Some(cmdb_env);
                Ok(())
            }
            ServiceID::SMDB => {
                let smdb_env = self.get_svc_env_config(ServiceID::SMDB, endpoint, metrics_config);
                *self.smdb_env.borrow_mut() = Some(smdb_env);
                Ok(())
            }
            ServiceID::DBGW => {
                let dbgw_env = self.get_svc_env_config(ServiceID::DBGW, endpoint, metrics_config);
                *self.dbgw_env.borrow_mut() = Some(dbgw_env);
                Ok(())
            }
            ServiceID::QDGW => {
                let qdgw_env = self.get_svc_env_config(ServiceID::QDGW, endpoint, metrics_config);
                *self.qdgw_env.borrow_mut() = Some(qdgw_env);
                Ok(())
            }
            ServiceID::VEX => {
                let qdgw_env = self.get_svc_env_config(ServiceID::VEX, endpoint, metrics_config);
                *self.vex_env.borrow_mut() = Some(qdgw_env);
                Ok(())
            }

            ServiceID::Default => Err(InitError(format!(
                "[EnvManager]: Service {:?} is not supported",
                svc_id
            ))),
        }
    }

    /// Returns true only if the service environment has been initialized.
    pub fn is_svc_env_initialized(&self, svc_id: &ServiceID) -> bool {
        match svc_id {
            ServiceID::CMDB => self.cmdb_env.borrow().is_some(),
            ServiceID::SMDB => self.smdb_env.borrow().is_some(),
            ServiceID::DBGW => self.dbgw_env.borrow().is_some(),
            ServiceID::QDGW => self.qdgw_env.borrow().is_some(),
            ServiceID::VEX => self.vex_env.borrow().is_some(),
            ServiceID::Default => false,
        }
    }

    /// returns the socket address to run the service in any context.
    pub fn configure_svc_socket_addr(&self, svc_id: &ServiceID) -> Result<String, InitError> {
        // Get the configuration of the service
        let svc_config = self
            .get_svc_env(svc_id)
            .expect("Failed to get service config");
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
    pub fn configure_metrics_socket_addr_uri(
        &self,
        svc_id: &ServiceID,
    ) -> Result<(String, String), InitError> {
        let (metrics_host, metrics_uri, metrics_port) = self
            .get_svc_metric_host_uri_port(svc_id)
            .expect("Failed to get metric host, uri, and port");

        // Merge the host and port into a socket address i.e. 0.0.0.0:8080
        let socket_addr = format!("{}:{}", metrics_host, metrics_port);

        Ok((socket_addr, metrics_uri))
    }

    /// Returns the host, uri, and port of the metrics endpoint of the service relative to its context.
    /// Use this to pull metrics data from the service regardless of deployment context.
    pub fn get_svc_metric_host_uri_port(
        &self,
        svc_id: &ServiceID,
    ) -> Result<(String, String, u16), InitError> {
        // Check if the service is initialized
        if !self.is_svc_env_initialized(svc_id) {
            InitError(format!(
                "[EnvManager:get_svc_metric_host_uri_port]: Service {:?} is not initialized",
                svc_id
            ));
        };

        let svc = self
            .get_svc_env(svc_id)
            .expect("Failed to get service environment");

        let metric_host = svc.metrics_host().to_string();
        let metrics_uri = svc.metrics_uri().to_string();
        let metrics_port = *svc.metrics_port();

        Ok((metric_host, metrics_uri, metrics_port))
    }

    ///  Returns the hostname of the service relative to the application context.
    ///  Use this to connect to the service regardless of deployment context.
    ///
    ///  If the environment type is local, it returns the hostname of the service running locally.
    ///  If the environment type is cluster, it returns the hostname of the service running in the cluster.
    ///  If the environment type is unknown, it returns an error.
    /// The function checks if the service is initialized, and if not, it returns an InitError.
    pub fn get_svc_host_port(&self, svc_id: &ServiceID) -> Result<(String, u16), InitError> {
        // Check if the service is initialized
        if !self.is_svc_env_initialized(svc_id) {
            InitError(format!(
                "[EnvManager:get_svc_host_port]: Service {:?} is not initialized",
                svc_id
            ));
        };
        // Get the configuration of the service
        let svc_config = self
            .get_svc_env(svc_id)
            .expect("Failed to get service config");
        // Get the host and port of the service
        self.get_host(&svc_config)
    }
}

impl<'l> CfgManager<'l> {
    // The functions take a HostEndpoint struct as an argument, which contains the hostname and port of the respective service.
    fn get_svc_env_config(
        &self,
        service_id: ServiceID,
        endpoint: HostEndpoint,
        metrics_config: MetricConfig,
    ) -> SvcEnvConfig {
        let local_host = "127.0.0.1".to_string();
        let cluster_host = endpoint.host_uri().to_string();
        let ci_host = "127.0.0.1".to_string();
        let docker_host = "0.0.0.0".to_string();
        let service_port = endpoint.port().to_string();
        let metrics_host = metrics_config.metric_host().to_string();
        let metrics_uri = metrics_config.metric_uri().to_string();
        let metrics_port = metrics_config.metric_port();

        SvcEnvConfig::new(
            service_id,
            cluster_host,
            ci_host,
            local_host,
            docker_host,
            service_port,
            metrics_host,
            metrics_uri,
            metrics_port,
        )
    }

    // Returns the hostname and port of the service based on the environment type.
    // If the environment type is local, it returns the hostname of the service running locally.
    // If the environment type is cluster, it returns the hostname of the service running in the cluster.
    // If the environment type is unknown, it returns an error.
    fn get_host(&self, svc_env_config: &SvcEnvConfig) -> Result<(String, u16), InitError> {
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

            EnvironmentType::Docker => svc_env_config.docker_host().to_string(),

            EnvironmentType::UnknownEnv => {
                return Err(InitError("[EnvManager]: Unknown Environment".to_string()));
            }
        };

        Ok((host, port))
    }

    fn get_svc_env(&self, svc_id: &ServiceID) -> Result<SvcEnvConfig, InitError> {
        match svc_id {
            ServiceID::CMDB => {
                let svc = self
                    .cmdb_env
                    .borrow()
                    .as_ref()
                    .expect("[EnvManager]: Failed to get cmdb host and port")
                    .to_owned();

                Ok(svc)
            }

            ServiceID::SMDB => {
                let svc = self
                    .smdb_env
                    .borrow()
                    .as_ref()
                    .expect("[EnvManager]: Failed to get smdb host and port")
                    .to_owned();

                Ok(svc)
            }

            ServiceID::DBGW => {
                let svc = self
                    .dbgw_env
                    .borrow()
                    .as_ref()
                    .expect("[EnvManager]: Failed to get dbgw host and port")
                    .to_owned();

                Ok(svc)
            }

            ServiceID::QDGW => {
                let svc = self
                    .qdgw_env
                    .borrow()
                    .as_ref()
                    .expect("[EnvManager]: Failed to get qdgw host and port")
                    .to_owned();

                Ok(svc)
            }

            ServiceID::VEX => {
                let svc = self
                    .vex_env
                    .borrow()
                    .as_ref()
                    .expect("[EnvManager]: Failed to get vex host and port")
                    .to_owned();

                Ok(svc)
            }

            ServiceID::Default => Err(InitError(format!(
                "[EnvManager]: Service {:?} is not supported",
                svc_id
            ))),
        }
    }
}
