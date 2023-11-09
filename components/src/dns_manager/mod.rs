/**
 * This file contains the implementation of the DnsManager, which is responsible for managing DNS resolution for both internal and external hosts.
 *
 * The DnsManager uses two Resolvers: an internal (cluster) DNS resolver and an external (Cloudflare) DNS resolver.
 * The internal resolver is used to resolve hostnames within the cluster,
 * while the external resolver is used to resolve hostnames outside of the cluster.
 *
 * The DnsManager provides a single method, resolve_dns(), which takes a host name and a boolean
 * indicating whether the hostname should be resolved using the internal or external resolver.
 * The method uses the appropriate resolver to perform the lookup and returns the first IP address found.
 *
 * The implementation is based on the Hickory DNS library.
 */
use std::net::{IpAddr, SocketAddr};

use hickory_resolver::config::*;
use hickory_resolver::error::ResolveError;
use hickory_resolver::Resolver;

use common::prelude::EnvironmentType;

use crate::prelude::CtxManager;

/**
 * The DnsManager struct contains two Resolvers, one for internal (cluster) DNS resolution
 * and one for external (Cloudflare) DNS resolution.
 */
pub struct DnsManager {
    internal_resolver: Resolver,
    internal_dns: String,
    external_resolver: Resolver,
    external_dns: String,
}

impl DnsManager {
    /**
     * Creates a new DnsManager instance.
     */
    pub fn new(ctx: &CtxManager) -> Self {
        // Build the external (Cloudflare) DNS address resolver
        let config = build_cloudflare_resolver_config();
        let external_dns = "1.1.1.1:53".to_string();
        let external_resolver = Resolver::new(config, ResolverOpts::default())
            .expect("Failed to construct external (Cloudflare) DNS resolver");

        // Build the internal cluster DNS resolver
        let internal_dns_host = match ctx.env_type() {
            EnvironmentType::LOCAL => "127.0.0.1",
            EnvironmentType::CI => "127.0.0.1",
            EnvironmentType::CLUSTER => match ctx.int_dns_server() {
                Some(cluster_dns_server) => cluster_dns_server,
                None => {
                    panic!("Failed to find cluster DNS_SERVER env. Ensure DNS_SERVER is set as environment variable in deployment.yaml");
                }
            },
            EnvironmentType::UnknownEnv => "1.1.1.1",
        };

        let mut internal_dns = internal_dns_host.to_string();
        internal_dns.push_str(":53");

        let config = build_custom_resolver_config(&internal_dns);
        let internal_resolver = Resolver::new(config, ResolverOpts::default())
            .expect("Failed to construct internal CLUSTER DNS resolver");

        Self {
            internal_resolver,
            internal_dns,
            external_resolver,
            external_dns,
        }
    }
}

// getters
impl DnsManager {
    /**
     * Returns a reference to the IP address of the internal DNS resolver.
     */
    pub fn internal_dns(&self) -> &str {
        &self.internal_dns
    }

    /**
     * Returns a reference to the IP address of the external DNS resolver.
     */
    pub fn external_dns(&self) -> &str {
        &self.external_dns
    }
}

impl DnsManager {
    /**
     * Resolves a hostname using the appropriate DNS resolver (internal or external).
     */
    pub fn resolve_dns(&self, host: &str, internal: bool) -> Result<IpAddr, ResolveError> {
        if internal {
            // resolve host name using the internal (cluster) DNS server
            resolve_address(&self.internal_resolver, host)
        } else {
            // resolve host name using the external DNS server
            resolve_address(&self.external_resolver, host)
        }
    }
}

/**
 * Resolves a hostname using the specified DNS resolver.
 */
fn resolve_address(resolver: &Resolver, host: &str) -> Result<IpAddr, ResolveError> {
    // resolve host name using the external DNS server
    let response = match resolver.lookup_ip(host) {
        Ok(response) => response,
        Err(e) => return Err(e),
    };

    match response.iter().next() {
        Some(address) => Ok(address),
        None => Err(ResolveError::from("Address not found")),
    }
}

/**
 * Builds a ResolverConfig for Cloudflare DNS resolution.
 */
fn build_cloudflare_resolver_config() -> ResolverConfig {
    ResolverConfig::cloudflare()
}

/**
 * Builds a ResolverConfig for a custom DNS resolution.
 */
fn build_custom_resolver_config(address: &str) -> ResolverConfig {
    let socket_addr: SocketAddr = match address.parse() {
        Ok(addr) => addr,
        Err(e) => panic!("Failed to parse DNS SERVER address: {}", e),
    };

    let protocol = Protocol::Udp;

    let name_server = NameServerConfig::new(socket_addr, protocol);

    let mut config = ResolverConfig::new();

    config.add_name_server(name_server);

    config
}
