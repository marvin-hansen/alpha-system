use std::fmt::Error;

use common::prelude::{HostEndpoint, ServiceID};

use crate::env_manager::env_config::SvcEnvConfig;
use crate::prelude::{CtxManager, DnsManager};

mod env_config;

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
    pub fn init_cmdb_env(&mut self, endpoint: HostEndpoint) -> Result<(), Error> {
        let cmdb_env = self.get_svc_env_config(ServiceID::CMDB, endpoint);
        self.cmdb_env = Some(cmdb_env);
        Ok(())
    }

    pub fn init_smdb_env(&mut self, endpoint: HostEndpoint) -> Result<(), Error> {
        let smdb_env = self.get_svc_env_config(ServiceID::SMDB, endpoint);
        self.smdb_env = Some(smdb_env);
        Ok(())
    }

    pub fn init_memgraph_env(&mut self, endpoint: HostEndpoint) -> Result<(), Error> {
        let memgraph_env = self.get_svc_env_config(ServiceID::MEMGRAPH, endpoint);
        self.memgraph_env = Some(memgraph_env);
        Ok(())
    }

    fn get_svc_env_config(&self, service_id: ServiceID, endpoint: HostEndpoint) -> SvcEnvConfig {
        let local_host = "localhost".to_string();
        let cluster_host = endpoint.host_uri().to_string();
        let port = endpoint.port().to_string();

        SvcEnvConfig::new(service_id, cluster_host, local_host, port)
    }
}