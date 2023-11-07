use std::fmt::{Display, Error, Formatter};

use common::prelude::{DnsConfig, EnvironmentType};

use crate::prelude::CtxManager;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct DnsManager {
    internal_dns: String,
    extern_dns: String,
}

impl DnsManager {
    pub fn new(dns_config: DnsConfig, ctx: &CtxManager) -> Self {
        // Build the external DNS address
        let external_dns_host = dns_config.dns_host_external();
        let external_dns_port = dns_config.dns_port_external();
        let extern_dns = format!("{}:{}", external_dns_host, external_dns_port);

        // Build the internal cluster DNS address
        let internal_dns_port = dns_config.dns_port_internal();
        let internal_dns_host = match ctx.env_type() {
            EnvironmentType::LOCAL => "127.0.0.1".to_string(),
            EnvironmentType::CLUSTER => match ctx.int_dns_server() {
                Some(cluster_dns_server) => cluster_dns_server,
                None => {
                    panic!("Failed to find cluster DNS_SERVER env. Ensure DNS_SERVER is set in deployment.yaml");
                }
            }.to_string(),
            EnvironmentType::UnknownEnv => "1.1.1.1".to_string(),
        };

        let internal_dns = format!("{}:{}", internal_dns_host, internal_dns_port);

        Self {
            internal_dns,
            extern_dns,
        }
    }
}

impl DnsManager {
    pub fn resolve_dns(&self, _hostname: &str, internal: bool) -> Result<String, Error> {
        let mut res = String::new();

        if internal {
            // resolve using the internal (cluster) DNS server
            res = self.internal_dns.clone();
        } else {
            // resolve using the external DNS server
            res = self.extern_dns.clone();
        }

        // Implement with dns resolver using a custom DNS server
        Ok(res)
    }
}

impl DnsManager {
    pub fn internal_dns(&self) -> &str {
        &self.internal_dns
    }
    pub fn extern_dns(&self) -> &str {
        &self.extern_dns
    }
}

impl Display for DnsManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DnsManager {{ internal_dns: {}, extern_dns: {} }}",
            self.internal_dns, self.extern_dns
        )
    }
}
