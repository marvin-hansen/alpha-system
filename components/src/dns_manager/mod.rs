use std::fmt::Error;

use common::prelude::{DnsConfig, EnvironmentType};

use crate::dns_manager::dns_manager_impl::{DnsManagerImpl, Traited};

pub mod dns_manager_impl;

pub fn new_dns_manager(dns_config: DnsConfig, env_type: EnvironmentType) -> Result<Traited, Error> {
    Ok(Traited::from(Traited::DnsManagerImpl(DnsManagerImpl::new(dns_config, env_type))))
}

pub trait DnsManager {
    fn resolve_cluster_dns(&self, hostname: &str) -> Result<String, Error>;
    fn resolve_remote_dns(&self, hostname: &str) -> Result<String, Error>;
}