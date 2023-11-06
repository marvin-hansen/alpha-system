use std::env;
use std::fmt::Error;

use enum_dispatch::enum_dispatch;

use common::prelude::{DnsConfig, EnvironmentType};

use crate::dns_manager::DnsManager;

#[enum_dispatch(DnsManager)]
pub enum Traited {
    DnsManagerImpl,
}


pub struct DnsManagerImpl {
    cluster_dns: String,
    extern_dns: String,
}

impl DnsManagerImpl {
    pub fn new(dns_config: DnsConfig, env_type: EnvironmentType) -> Self {

        // Build the external DNS address
        let external_dns_host = dns_config.dns_host_external();
        let external_dns_port = dns_config.dns_port_external();
        let extern_dns = format!("{}{}", external_dns_host, external_dns_port);

        // Build the internal cluster DNS address
        let internal_dns_port = dns_config.dns_port_internal();
        let internal_dns_host = match env_type {
            EnvironmentType::LOCAL => "127.0.0.1".to_string(),
            EnvironmentType::CLUSTER => match env::var("DNS_SERVER") {
                Ok(cluster_dns) => cluster_dns,
                Err(e) => panic!("Failed to read DNS_SERVER env. Ensure DNS_SERVER is set in deployment.yaml:m {}", e),
            },
            EnvironmentType::UnknownEnv => "127.0.0.1".to_string(),
        };

        let cluster_dns = format!("{}:{}", internal_dns_host, internal_dns_port);

        Self {
            cluster_dns,
            extern_dns,
        }
    }
}

impl DnsManager for DnsManagerImpl {
    fn resolve_cluster_dns(&self, hostname: &str) -> Result<String, Error> {
        self.resolve_dns(hostname, true)
    }

    fn resolve_remote_dns(&self, hostname: &str) -> Result<String, Error> {
        self.resolve_dns(hostname, false)
    }
}

impl DnsManagerImpl {
    fn resolve_dns(&self, hostname: &str, cluster: bool) -> Result<String, Error> {
        Ok("127.0.0.1:53".to_string())
    }
}
