use std::cell::RefCell;

use common::prelude::{EnvironmentType, HostEndpoint, InitError, ServiceID, SvcEnvConfig};

use crate::prelude::{CtxManager, DnsManager};

pub struct EnvManager<'l> {
    ctx_manager: &'l CtxManager,
    dns_manager: &'l DnsManager,
    cmdb_env: RefCell<Option<SvcEnvConfig>>,
    smdb_env: RefCell<Option<SvcEnvConfig>>,
    dbgw_env: RefCell<Option<SvcEnvConfig>>,
    qdgw_env: RefCell<Option<SvcEnvConfig>>,
}

impl<'l> EnvManager<'l> {
    pub fn new(ctx_manager: &'l CtxManager, dns_manager: &'l DnsManager) -> Self {
        Self {
            ctx_manager,
            dns_manager,
            cmdb_env: RefCell::new(None),
            smdb_env: RefCell::new(None),
            dbgw_env: RefCell::new(None),
            qdgw_env: RefCell::new(None),
        }
    }
}

impl<'l> EnvManager<'l> {
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
    ) -> Result<(), InitError> {
        match svc_id {
            ServiceID::CMDB => {
                let cmdb_env = self.get_svc_env_config(ServiceID::CMDB, endpoint);
                *self.cmdb_env.borrow_mut() = Some(cmdb_env);
                Ok(())
            }
            ServiceID::SMDB => {
                let smdb_env = self.get_svc_env_config(ServiceID::SMDB, endpoint);
                *self.smdb_env.borrow_mut() = Some(smdb_env);
                Ok(())
            }
            ServiceID::DBGW => {
                let dbgw_env = self.get_svc_env_config(ServiceID::DBGW, endpoint);
                *self.dbgw_env.borrow_mut() = Some(dbgw_env);
                Ok(())
            }
            ServiceID::QDGW => {
                let qdgw_env = self.get_svc_env_config(ServiceID::QDGW, endpoint);
                *self.qdgw_env.borrow_mut() = Some(qdgw_env);
                Ok(())
            }
            ServiceID::Default => Err(InitError(format!(
                "[EnvManager]: Service {:?} is not supported",
                svc_id
            ))),
        }
    }

    // The functions take a HostEndpoint struct as an argument, which contains the hostname and port of the respective service.
    fn get_svc_env_config(&self, service_id: ServiceID, endpoint: HostEndpoint) -> SvcEnvConfig {
        let local_host = "127.0.0.1".to_string();
        let cluster_host = endpoint.host_uri().to_string();
        let ci_host = "127.0.0.1".to_string();
        let port = endpoint.port().to_string();
        SvcEnvConfig::new(service_id, cluster_host, ci_host, local_host, port)
    }
}

impl<'l> EnvManager<'l> {
    ///  Returns the hostname of the service relative to the application context.
    ///  If the environment type is local, it returns the hostname of the service running locally.
    ///  If the environment type is cluster, it returns the hostname of the service running in the cluster.
    ///  If the environment type is unknown, it returns an error.
    /// The function checks if the service is initialized, and if not, it returns an InitError.
    pub fn get_svc_host_port(&self, svc_id: ServiceID) -> Result<(String, u16), InitError> {
        match svc_id {
            ServiceID::CMDB => {
                self.is_svc_env_initialized(&svc_id);
                self.get_host(
                    self.cmdb_env
                        .borrow()
                        .as_ref()
                        .expect("[EnvManager]: Failed to get cmdb host and port"),
                )
            }
            ServiceID::SMDB => {
                self.is_svc_env_initialized(&svc_id);
                self.get_host(
                    self.smdb_env
                        .borrow()
                        .as_ref()
                        .expect("[EnvManager]: Failed to get smdb host and port"),
                )
            }
            ServiceID::DBGW => {
                self.is_svc_env_initialized(&svc_id);
                self.get_host(
                    self.dbgw_env
                        .borrow()
                        .as_ref()
                        .expect("[EnvManager]: Failed to get dbgw host and port"),
                )
            }
            ServiceID::QDGW => {
                self.is_svc_env_initialized(&svc_id);
                self.get_host(
                    self.qdgw_env
                        .borrow()
                        .as_ref()
                        .expect("[EnvManager]: Failed to get qdgw host and port"),
                )
            }
            ServiceID::Default => Err(InitError(format!(
                "[EnvManager]: Service {:?} is not supported",
                svc_id
            ))),
        }
    }

    pub fn is_svc_env_initialized(&self, svc_id: &ServiceID) -> bool {
        match svc_id {
            ServiceID::CMDB => self.cmdb_env.borrow().is_some(),
            ServiceID::SMDB => self.smdb_env.borrow().is_some(),
            ServiceID::DBGW => self.dbgw_env.borrow().is_some(),
            ServiceID::QDGW => self.qdgw_env.borrow().is_some(),
            ServiceID::Default => false,
        }
    }

    // Returns the hostname and port of the service based on the environment type.
    // If the environment type is local, it returns the hostname of the service running locally.
    // If the environment type is cluster, it returns the hostname of the service running in the cluster.
    // If the environment type is unknown, it returns an error.
    fn get_host(&self, svc_env_config: &SvcEnvConfig) -> Result<(String, u16), InitError> {
        let port: u16 = svc_env_config
            .port()
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
