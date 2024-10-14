use cmdb_specs::cmdb_service_config;
use common_config::prelude::ServiceID::{CMDB, DBGW, SMDB};
use common_config::prelude::{ServiceID, SvcEnvConfig};
use common_env::prelude::EnvironmentType;
use common_errors::prelude::InitError;
use dbgw_specs::dbgw_service_config;
use smdb_specs::smdb_service_config;

use crate::build_utils as utils;
use crate::{CfgManager, DEFAULT_HOST};

impl CfgManager {
    /// Returns the host and port of the service.
    ///
    /// # Returns
    ///
    /// A tuple containing the host and port of the service as a string and u16, respectively. Returns an error if the
    /// host and port cannot be obtained.
    pub async fn get_smdb_host_port(&self) -> Result<(String, u16), InitError> {
        self.dbg_print("get_smdb_host_port");

        self.dbg_print("Get SMDB service configuration!");
        let svc_config = smdb_service_config();

        self.dbg_print("Construct contextual service environment configuration");
        let svc_env_config = utils::get_svc_env_config(self.dbg, SMDB, &svc_config);

        self.dbg_print("Get the host and port of the service");
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
        self.dbg_print("get_cmdb_host_port");

        self.dbg_print("Get CMDB service configuration!");
        let svc_config = cmdb_service_config();

        self.dbg_print("Construct contextual service environment configuration");
        let svc_env_config = utils::get_svc_env_config(self.dbg, CMDB, &svc_config);

        self.dbg_print("Get the host and port of the service");
        self.get_host(&svc_env_config).await
    }

    /// Returns the host and port of the DBGW service.
    ///
    /// # Returns
    ///
    /// A tuple containing the host and port of the DBGW service as a string and u16, respectively. Returns an error if the
    /// host and port cannot be obtained.
    pub async fn get_dbgw_host_port(&self) -> Result<(String, u16), InitError> {
        self.dbg_print("get_dbgw_host_port");

        self.dbg_print("Get DBGW service configuration!");
        let svc_config = dbgw_service_config();

        self.dbg_print("Construct contextual service environment configuration");
        let svc_env_config = utils::get_svc_env_config(self.dbg, DBGW, &svc_config);

        self.dbg_print("Get the host and port of the service");
        self.get_host(&svc_env_config).await
    }

    /// Returns the host and port of the service.
    ///
    /// # Returns
    ///
    /// A tuple containing the host and port of the service as a string and u16, respectively. Returns an error if the
    /// host and port cannot be obtained.
    pub async fn get_service_host_port(&self) -> Result<(String, u16), InitError> {
        self.dbg_print("get_service_host_port");

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
        self.dbg_print("get_service_dependencies");

        self.get_svc_config().dependencies().clone()
    }

    /// returns the socket address to run the service in any context.
    pub async fn get_svc_socket_addr(&self) -> Result<String, InitError> {
        self.dbg_print("get_svc_socket_addr");

        // Get the configuration of the service
        let svc_config = self.svc_env_config.to_owned();
        // Get the host and port of the service
        let svc_port: u16 = svc_config
            .service_port()
            .parse()
            .expect("[EnvManager]: Failed to parse port from config");

        let port = self
            .get_port(svc_port, &svc_config.service_id())
            .expect("[EnvManager]: Failed to get port from config");

        // Set host to default (0.0.0.0) to listen on all interfaces
        // Merge the host and port into a socket address i.e. 0.0.0.0:7070
        let socket_addr = format!("{}:{}", DEFAULT_HOST, port);

        Ok(socket_addr)
    }

    // Returns the hostname and port of the service based on the environment type.
    // If the environment type is local, it returns the hostname of the service running locally.
    // If the environment type is cluster, it returns the hostname of the service running in the cluster.
    // If the environment type is unknown, it returns an error.
    pub(crate) async fn get_host(
        &self,
        svc_env_config: &SvcEnvConfig,
    ) -> Result<(String, u16), InitError> {
        self.dbg_print("get_host");

        //
        let svc_port: u16 = svc_env_config
            .service_port()
            .parse()
            .expect("[EnvManager]: Failed to parse port from config");

        let port = self
            .get_port(svc_port, &svc_env_config.service_id())
            .expect("[EnvManager]: Failed to get port from config");

        let host = match self.get_service_host().await {
            Ok(host) => host,

            Err(err) => {
                return Err(err);
            }
        };

        Ok((host, port))
    }

    pub(crate) async fn get_service_host(
        &self,
        svc_env_config: &SvcEnvConfig,
    ) -> Result<String, InitError> {
        self.dbg_print("get_service_host");
        self.dbg_print("EnvironmentType");
        self.dbg_print(self.env_type.to_string().as_str());

        match self.env_type {
            EnvironmentType::LOCAL => Ok(svc_env_config.local_host().to_string()),

            EnvironmentType::CI => Ok(svc_env_config.ci_host().to_string()),

            EnvironmentType::CLUSTER => {
                let cluster_host = self
                    .resolve_dns(svc_env_config.cluster_host(), true)
                    .await
                    .expect("[EnvManager]: Failed to resolve DNS");

                Ok(cluster_host.to_string())
            }

            EnvironmentType::UNKNOWN => {
                Err(InitError("[EnvManager]: Unknown Environment".to_string()))
            }
        }
    }

    /// Returns the port of the service based on the environment type.
    ///
    /// If the environment type is local, it returns the port of the service running locally.
    /// If the environment type is cluster, it returns the port of the service running in the cluster.
    /// If the environment type is unknown, it returns an error.
    ///
    pub(crate) fn get_port(&self, svc_port: u16, service_id: &ServiceID) -> Result<u16, InitError> {
        self.dbg_print("get_port");
        self.dbg_print("env_type");
        self.dbg_print(self.env_type.to_string().as_str());

        let port = match self.env_type {
            EnvironmentType::LOCAL => svc_port + service_id.as_u16(),
            EnvironmentType::CLUSTER => svc_port,
            EnvironmentType::CI => svc_port + service_id.as_u16(),
            EnvironmentType::UNKNOWN => svc_port,
        };

        Ok(port)
    }
}
