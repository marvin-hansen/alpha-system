/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::fields::{DEFAULT_DNS, DEFAULT_HOST};
use common_config::{ServiceConfig, ServiceID, SvcEnvConfig};
use common_env::EnvironmentType;
use hickory_resolver::config::*;
use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::proto::xfer::Protocol;
use hickory_resolver::{Resolver, TokioResolver};
use std::net::SocketAddr;

///
/// Builds an external DNS resolver using the Tokio async resolver.
///
/// # Arguments
///
/// * `dbg` - A boolean flag indicating whether to enable debug mode.
///
/// # Returns
///
/// A `TokioAsyncResolver` configured with Cloudflare resolver settings.
///
pub fn build_external_dns_resolver(dbg: bool) -> TokioResolver {
    if dbg {
        println!("[CfgManager]: build_external_dns_resolver");
    }

    let resolver = Resolver::builder_with_config(
        ResolverConfig::cloudflare(),
        TokioConnectionProvider::default(),
    )
    .build();

    resolver
}

///
/// Builds an internal DNS resolver using the provided address.
///
/// # Arguments
///
/// * `dbg` - A boolean indicating whether to print debug information.
/// * `address` - A string slice representing the address of the DNS server.
///
/// # Returns
///
/// A `TokioAsyncResolver` configured with the specified address and default resolver options.
///
pub fn build_internal_dns_resolver(dbg: bool, address: &str) -> TokioResolver {
    if dbg {
        println!("[CfgManager]: build_internal_dns_resolver");
    }

    let socket_addr: SocketAddr = match address.parse() {
        Ok(addr) => addr,
        Err(e) => panic!("Failed to parse DNS SERVER address: {e}"),
    };

    let name_server = NameServerConfig::new(socket_addr, Protocol::Udp);
    let mut config = ResolverConfig::new();
    config.add_name_server(name_server);

    let resolver =
        Resolver::builder_with_config(config, TokioConnectionProvider::default()).build();

    resolver
}

///
/// Builds the internal DNS server address based on the environment type.
///
/// If `dbg` is true, prints a debug message.
///
/// # Arguments
///
/// - `dbg`: A boolean indicating whether to print debug messages.
/// - `env_type`: A reference to the environment type.
///
/// # Returns
///
/// A string representing the internal DNS server address.
///
pub fn get_internal_dns_server(dbg: bool, env_type: &EnvironmentType) -> String {
    if dbg {
        println!("[CfgManager]: get_internal_dns_server");
    }

    // Find the internal DNS server based on the env context
    let internal_dns_host = match env_type {
        EnvironmentType::LOCAL => DEFAULT_DNS.to_owned(),
        EnvironmentType::CI => DEFAULT_DNS.to_owned(),
        EnvironmentType::CLUSTER => get_value_from_env("DNS_SERVER"),
        _ => DEFAULT_DNS.to_owned(),
    };

    // Build the internal DNS resolver to resolve hosts within the system network
    format!("{}{}", internal_dns_host, ":53")
}

///
/// Retrieves a value from the environment variables based on the provided key.
///
/// # Arguments
///
/// * `key` - A string slice that represents the key of the environment variable to retrieve.
///
/// # Returns
///
/// A `String` containing the value of the environment variable associated with the provided key.
///
/// # Panics
///
/// Panics if there is an error while retrieving the environment variable, providing a descriptive error message.
///
pub fn get_value_from_env(key: &str) -> String {
    match std::env::var(key) {
        Ok(cluster_dns_server) => cluster_dns_server,
        Err(e) => {
            panic!(
                "{} {}",
                format_args!("Failed to read {key} environment variable. Ensure {key} is set:"),
                e
            );
        }
    }
}

///
/// Retrieves the service environment configuration based on the provided parameters.
///
/// # Arguments
///
/// * `dbg` - A boolean indicating whether debug mode is enabled.
/// * `service_id` - The unique identifier of the service.
/// * `service_config` - A reference to the service configuration.
///
/// # Returns
///
/// A `SvcEnvConfig` object containing the service environment configuration.
///
pub fn get_svc_env_config(
    dbg: bool,
    service_id: ServiceID,
    service_config: &ServiceConfig,
) -> SvcEnvConfig {
    if dbg {
        println!("[CfgManager]: get_svc_env_config");
    }

    let binding = service_config.service_endpoint();
    let endpoint = binding.host_endpoint();
    let metrics_config = service_config.metrics_endpoint();
    let local_host = DEFAULT_HOST.to_string();
    let cluster_host = service_config.cluster_uri().to_string();
    let ci_host = "127.0.0.1".to_string();
    let docker_host = DEFAULT_HOST.to_string();
    let service_port = endpoint.port().to_string();
    let metrics_host = metrics_config.host().to_string();
    let metrics_uri = metrics_config.uri().to_string();
    let metrics_port = metrics_config.port();

    SvcEnvConfig::new(
        service_id,
        cluster_host,
        ci_host,
        local_host,
        docker_host,
        service_port,
        metrics_host,
        metrics_uri,
        metrics_port,
    )
}
