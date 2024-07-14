use common_config::prelude::EnvironmentType;
use hickory_resolver::config::*;
use hickory_resolver::Resolver;
use mini_moka::sync::Cache;
use std::env;
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
use std::time::Duration;

use ctx_manager::CtxManager;

mod display;
mod getters;
mod resolve_dns;

const DEFAULT_DNS: &str = "1.1.1.1";

/**
 * The DnsManager struct contains two Resolvers, one for internal (cluster) DNS resolution
 * and one for external (Cloudflare) DNS resolution.
 */
pub struct DnsManager {
    env_type: EnvironmentType,
    internal_dns_cache: Cache<String, IpAddr>,
    internal_dns_resolver: Resolver,
    internal_dns_server: String,
    external_dns_cache: Cache<String, IpAddr>,
    external_dns_resolver: Resolver,
    external_dns_server: String,
}

impl DnsManager {
    /**
     * Creates a new DnsManager instance.
     */
    pub fn new(ctx: &CtxManager) -> Self {
        let env_type = ctx.env_type();

        // Find the internal DNS server based on the env context
        let internal_dns_host = match env_type {
            EnvironmentType::LOCAL => Self::get_ci_local_dns(),
            EnvironmentType::CI => Self::get_ci_context_dns(),
            EnvironmentType::CLUSTER => Self::get_cluster_context_dns(),
            EnvironmentType::UNKNOWN => DEFAULT_DNS.to_owned(),
        };

        // Build the internal DNS resolver to resolve hosts within the system network
        let internal_dns_server = format!("{}{}", internal_dns_host, ":53");
        let internal_resolver_config = build_custom_resolver_config(&internal_dns_server);
        let internal_dns_resolver =
            Resolver::new(internal_resolver_config, ResolverOpts::default())
                .expect("Failed to construct internal CLUSTER DNS resolver");

        // Build the external (Cloudflare) DNS address resolver to resolve hosts on the open internet
        let external_dns_server = "1.1.1.1:53".to_string();
        let external_resolver_config = build_cloudflare_resolver_config();
        let external_dns_resolver =
            Resolver::new(external_resolver_config, ResolverOpts::default())
                .expect("Failed to construct internal CLUSTER DNS resolver");

        // Build DNS cache to speed up internal and external DNS lookups
        let internal_dns_cache = Self::get_dns_cache();
        let external_dns_cache = Self::get_dns_cache();

        Self {
            env_type,
            internal_dns_cache,
            internal_dns_resolver,
            internal_dns_server,
            external_dns_cache,
            external_dns_resolver,
            external_dns_server,
        }
    }

    fn get_ci_local_dns() -> String {
        DEFAULT_DNS.to_owned()
    }

    fn get_ci_context_dns() -> String {
        DEFAULT_DNS.to_owned()
    }
    fn get_cluster_context_dns() -> String {
        match env::var("DNS_SERVER") {
            Ok(cluster_dns_server) => cluster_dns_server,
            Err(e) => {
                panic!(
                    "Failed to read DNS_SERVER environment variable. Ensure DNS_SERVER is set in deployment.yaml:{}",
                    e
                );
            }
        }
    }

    fn get_dns_cache() -> Cache<String, IpAddr> {
        Cache::builder()
            // Time to live (TTL): 60 minutes
            .time_to_live(Duration::from_secs(60 * 60))
            // Time to idle (TTI):  15 minutes
            .time_to_idle(Duration::from_secs(15 * 60))
            // This cache will hold up to 4MiB of values.
            .max_capacity(4 * 1024 * 1024)
            // Create the cache.
            .build()
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

    let name_server = NameServerConfig::new(socket_addr, Protocol::Udp);

    let mut config = ResolverConfig::new();

    config.add_name_server(name_server);

    config
}
