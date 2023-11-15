use std::cell::RefCell;

use common::prelude::{EnvironmentType, HostEndpoint, InitError, ServiceID, SvcEnvConfig};

use crate::prelude::{CtxManager, DnsManager};

pub struct SvcEnvManager<'l> {
    ctx_manager: &'l CtxManager,
    dns_manager: &'l DnsManager,
    // interior mutability to keep the public API immutable
    // https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
    cmdb_env: RefCell<Option<SvcEnvConfig>>,
    smdb_env: RefCell<Option<SvcEnvConfig>>,
    memgraph_env: RefCell<Option<SvcEnvConfig>>,
}

impl<'l> SvcEnvManager<'l> {
    pub fn new(ctx_manager: &'l CtxManager, dns_manager: &'l DnsManager) -> Self {
        Self {
            ctx_manager,
            dns_manager,
            cmdb_env: RefCell::new(None),
            smdb_env: RefCell::new(None),
            memgraph_env: RefCell::new(None),
        }
    }
}

impl<'l> SvcEnvManager<'l> {
    pub fn is_svc_env_initialized(&self, svc_id: ServiceID) -> bool {
        match svc_id {
            ServiceID::CMDB => self.cmdb_env.borrow().is_some(),
            ServiceID::SMDB => self.smdb_env.borrow().is_some(),
            ServiceID::MEMGRAPH => self.memgraph_env.borrow().is_some(),
            ServiceID::Default => false,
        }
    }

    /// Initializes the service environment based on the given service ID and host endpoint.
    ///
    /// # Arguments
    ///
    /// * `svc_id` - The service ID of the service to be initialized.
    /// * `endpoint` - The host endpoint of the service.
    ///
    /// # Returns
    ///
    /// `Result<(), InitError>` containing a
    /// * `InitError` in case of an error
    /// * `Ok(())` if the service environment was successfully initialized.
    pub fn init_svc_env(&self, svc_id: &ServiceID, endpoint: HostEndpoint) -> Result<(), InitError> {
        match svc_id {
            ServiceID::CMDB => self.init_cmdb_env(endpoint),
            ServiceID::SMDB => self.init_smdb_env(endpoint),
            ServiceID::MEMGRAPH => self.init_memgraph_env(endpoint),
            ServiceID::Default => Err(InitError(format!("Service {:?} is not supported", svc_id))),
        }
    }

    // initializes CMDB. The cmdb_env field stores the configuration for the CMDB service.
    fn init_cmdb_env(&self, endpoint: HostEndpoint) -> Result<(), InitError> {
        let cmdb_env = self.get_svc_env_config(ServiceID::CMDB, endpoint);
        *self.cmdb_env.borrow_mut() = Some(cmdb_env);
        Ok(())
    }

    // initializes SMDB. The smdb_env field stores the configuration for the SMDB service.
    fn init_smdb_env(&self, endpoint: HostEndpoint) -> Result<(), InitError> {
        let smdb_env = self.get_svc_env_config(ServiceID::SMDB, endpoint);
        *self.smdb_env.borrow_mut() = Some(smdb_env);
        Ok(())
    }

    // initializes MEMGRAPH. The memgraph_env field stores the configuration for the MEMGRAPH service.
    fn init_memgraph_env(&self, endpoint: HostEndpoint) -> Result<(), InitError> {
        let memgraph_env = self.get_svc_env_config(ServiceID::MEMGRAPH, endpoint);
        *self.memgraph_env.borrow_mut() = Some(memgraph_env);
        Ok(())
    }

    // Initializes SvcEnvConfig fields.
    // The functions take a HostEndpoint struct as an argument, which contains the hostname and port of the respective service.
    fn get_svc_env_config(&self, service_id: ServiceID, endpoint: HostEndpoint) -> SvcEnvConfig {
        let local_host = "127.0.0.1".to_string();
        let cluster_host = endpoint.host_uri().to_string();
        let ci_host = "127.0.0.1".to_string();
        let port = endpoint.port().to_string();

        SvcEnvConfig::new(service_id, cluster_host, ci_host, local_host, port)
    }
}

impl<'l> SvcEnvManager<'l> {
    ///  Returns the hostname of the service relative to the application context.
    ///  If the environment type is local, it returns the hostname of the service running locally.
    ///  If the environment type is cluster, it returns the hostname of the service running in the cluster.
    ///  If the environment type is unknown, it returns an error.
    /// The function checks if the service is initialized, and if not, it returns an InitError.
    pub fn get_svc_host_port(&self, svc_id: ServiceID) -> Result<(String, u16), InitError> {
        match svc_id {
            ServiceID::CMDB => self.get_cmdb_host(),
            ServiceID::SMDB => self.get_smdb_host(),
            ServiceID::MEMGRAPH => self.get_memgraph_host(),
            ServiceID::Default => Err(InitError(format!("Service {:?} is not supported", svc_id))),
        }
    }

    fn get_cmdb_host(&self) -> Result<(String, u16), InitError> {
        if self.cmdb_env.borrow().is_none() {
            Err(InitError(
                "CMDB Env. Not Initialized. Call init_cmdb_env()".to_string(),
            ))
        } else {
            self.get_host(self.cmdb_env.borrow().as_ref().unwrap())
        }
    }

    fn get_smdb_host(&self) -> Result<(String, u16), InitError> {
        if self.smdb_env.borrow().is_none() {
            Err(InitError(
                "CMDB Env. Not Initialized. Call init_smdb_env()".to_string(),
            ))
        } else {
            self.get_host(self.smdb_env.borrow().as_ref().unwrap())
        }
    }

    fn get_memgraph_host(&self) -> Result<(String, u16), InitError> {
        if self.memgraph_env.borrow().is_none() {
            Err(InitError(
                "CMDB Env. Not Initialized. Call init_memgraph_env()".to_string(),
            ))
        } else {
            self.get_host(self.memgraph_env.borrow().as_ref().unwrap())
        }
    }


    // Returns the hostname and port of the service based on the environment type.
    // If the environment type is local, it returns the hostname of the service running locally.
    // If the environment type is cluster, it returns the hostname of the service running in the cluster.
    // If the environment type is unknown, it returns an error.
    fn get_host(&self, svc_env_config: &SvcEnvConfig) -> Result<(String, u16), InitError> {
        let port: u16 = svc_env_config.port().parse().unwrap();

        let host = match self.ctx_manager.env_type()
        {
            EnvironmentType::LOCAL => {
                svc_env_config.local_host().to_string()
            }

            EnvironmentType::CI => {
                svc_env_config.ci_host().to_string()
            }

            EnvironmentType::CLUSTER => {
                let cluster_host = self.dns_manager
                    .resolve_dns(svc_env_config.cluster_host(), true)
                    .expect("Failed to resolve DNS");

                cluster_host.to_string()
            }
            EnvironmentType::UnknownEnv => {
                svc_env_config.local_host().to_string()
            }
        };

        Ok((host, port))
    }
}
