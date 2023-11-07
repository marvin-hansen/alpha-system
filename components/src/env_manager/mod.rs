use std::fmt::Error;

use common::prelude::{EnvironmentType, HostEndpoint, InitError, ServiceID, SvcEnvConfig};

use crate::prelude::{CtxManager, DnsManager};

pub struct EnvironmentManager<'l> {
    ctx_manager: &'l CtxManager,
    dns_manager: &'l DnsManager,
    cmdb_env: Option<SvcEnvConfig>,
    smdb_env: Option<SvcEnvConfig>,
    memgraph_env: Option<SvcEnvConfig>,
}

impl<'l> EnvironmentManager<'l> {
    pub fn new(ctx_manager: &'l CtxManager, dns_manager: &'l DnsManager) -> Self {
        Self { ctx_manager, dns_manager, cmdb_env: None, smdb_env: None, memgraph_env: None }
    }
}

impl<'l> EnvironmentManager<'l> {
    /// initializes CMDB. The cmdb_env field stores the configuration for the CMDB service.
    pub fn init_cmdb_env(&mut self, endpoint: HostEndpoint) -> Result<(), Error> {
        let cmdb_env = self.get_svc_env_config(ServiceID::CMDB, endpoint);
        self.cmdb_env = Some(cmdb_env);
        Ok(())
    }

    /// initializes SMDB. The smdb_env field stores the configuration for the SMDB service.
    pub fn init_smdb_env(&mut self, endpoint: HostEndpoint) -> Result<(), Error> {
        let smdb_env = self.get_svc_env_config(ServiceID::SMDB, endpoint);
        self.smdb_env = Some(smdb_env);
        Ok(())
    }

    /// initializes MEMGRAPH. The memgraph_env field stores the configuration for the MEMGRAPH service.
    pub fn init_memgraph_env(&mut self, endpoint: HostEndpoint) -> Result<(), Error> {
        let memgraph_env = self.get_svc_env_config(ServiceID::MEMGRAPH, endpoint);
        self.memgraph_env = Some(memgraph_env);
        Ok(())
    }

    // Initializes SvcEnvConfig fields.
    // The functions take a HostEndpoint struct as an argument, which contains the hostname and port of the respective service.
    fn get_svc_env_config(&self, service_id: ServiceID, endpoint: HostEndpoint) -> SvcEnvConfig {
        let local_host = "localhost".to_string();
        let cluster_host = endpoint.host_uri().to_string();
        let port = endpoint.port().to_string();

        SvcEnvConfig::new(service_id, cluster_host, local_host, port)
    }
}

impl<'l> EnvironmentManager<'l> {
    ///  Returns the hostname of the CMDB service relative to the application context.
    ///  If the environment type is local, it returns the hostname of the service running locally.
    ///  If the environment type is cluster, it returns the hostname of the service running in the cluster.
    ///  If the environment type is unknown, it returns an error.
    /// The function checks if the cmdb_env field is initialized, and if not, it returns an InitError.
    pub fn get_cmdb_host(&self) -> Result<String, InitError> {
        if self.cmdb_env.is_none() {
            return Err(InitError("CMDB Env. Not Initialized. Call init_cmdb_env()".to_string()));
        } else {
            let svc_env_config = self.cmdb_env.as_ref().unwrap();
            self.get_host(svc_env_config)
        }
    }

    ///  Returns the hostname of the SMDB service relative to the application context.
    ///  If the environment type is local, it returns the hostname of the service running locally.
    ///  If the environment type is cluster, it returns the hostname of the service running in the cluster.
    ///  If the environment type is unknown, it returns an error.
    /// The function checks if the smdb_env field is initialized, and if not, it returns an InitError.
    pub fn get_smdb_host(&self) -> Result<String, InitError> {
        if self.smdb_env.is_none() {
            return Err(InitError("CMDB Env. Not Initialized. Call init_smdb_env()".to_string()));
        } else {
            let svc_env_config = self.smdb_env.as_ref().unwrap();
            self.get_host(svc_env_config)
        }
    }

    ///  Returns the hostname of the MEMGRAPH service relative to the application context.
    ///  If the environment type is local, it returns the hostname of the service running locally.
    ///  If the environment type is cluster, it returns the hostname of the service running in the cluster.
    ///  If the environment type is unknown, it returns an error.
    /// The function checks if the memgraph_env field is initialized, and if not, it returns an InitError.
    pub fn get_memgraph_host(&self) -> Result<String, InitError> {
        if self.memgraph_env.is_none() {
            return Err(InitError("CMDB Env. Not Initialized. Call init_memgraph_env()".to_string()));
        } else {
            let svc_env_config = self.memgraph_env.as_ref().unwrap();
            self.get_host(svc_env_config)
        }
    }

    // Returns the hostname of the service based on the environment type.
    // If the environment type is local, it returns the hostname of the service running locally.
    // If the environment type is cluster, it returns the hostname of the service running in the cluster.
    // If the environment type is unknown, it returns an error.
    fn get_host(&self, svc_env_config: &SvcEnvConfig) -> Result<String, InitError> {
        let host = match self.ctx_manager.env_type() {
            EnvironmentType::LOCAL => {
                Ok(svc_env_config.local_host().to_string())
            }
            EnvironmentType::CLUSTER => {
                let port = svc_env_config.port();
                let host = match self.dns_manager.resolve_dns(
                    svc_env_config.cluster_host(),
                    true,
                ) {
                    Ok(host) => host,
                    Err(e) => return Err(InitError(e.to_string()))
                };

                let host_uri = format!("{}:{}", host, port);

                Ok(host_uri)
            }
            EnvironmentType::UnknownEnv => {
                Err(InitError("Unknown Environment".to_string()))
            }
        }.expect("Failed to get host");

        Ok(host.to_string())
    }
}