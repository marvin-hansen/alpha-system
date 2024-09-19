use crate::DEFAULT_DNS;
use common_config::prelude::{ServiceConfig, ServiceID, SvcEnvConfig};
use common_database::prelude::{ClickHouseConfig, PostgresDBConfig};
use common_env::prelude::EnvironmentType;
use db_specs_clickhouse::clickhouse;
use db_specs_postgres::postgres;
use hickory_resolver::config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts};
use hickory_resolver::TokioAsyncResolver;
use std::net::SocketAddr;

pub(super) fn build_external_dns_resolver(dbg: bool) -> TokioAsyncResolver {
    if dbg {
        println!("[CfgManager]: build_external_dns_resolver");
    }

    let external_resolver_config = ResolverConfig::cloudflare();
    TokioAsyncResolver::tokio(external_resolver_config, ResolverOpts::default())
}

pub(super) fn build_internal_dns_resolver(dbg: bool, address: &str) -> TokioAsyncResolver {
    if dbg {
        println!("[CfgManager]: build_internal_dns_resolver");
    }

    let socket_addr: SocketAddr = match address.parse() {
        Ok(addr) => addr,
        Err(e) => panic!("Failed to parse DNS SERVER address: {}", e),
    };

    let name_server = NameServerConfig::new(socket_addr, Protocol::Udp);
    let mut config = ResolverConfig::new();
    config.add_name_server(name_server);

    TokioAsyncResolver::tokio(config.clone(), ResolverOpts::default())
}

pub(super) fn build_internal_dns_server(dbg: bool, env_type: &EnvironmentType) -> String {
    if dbg {
        println!("[CfgManager]: build_internal_dns_resolver");
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

pub(super) fn get_clickhouse_config(dbg: bool, env_type: &EnvironmentType) -> ClickHouseConfig {
    if dbg {
        println!("[CfgManager]: get_clickhouse_config");
    }
    clickhouse::get_clickhouse_config(&env_type)
}

///
/// Returns the Postgres database configuration based on the debug mode and environment type.
///
/// If `dbg` is true, prints a debug message.
/// If the `env_type` is `EnvironmentType::CLUSTER`, retrieves the cluster database configuration
/// using environment variables defined in 'delivery/postgres/cluster.yaml'.
/// Otherwise, retrieves the default Postgres database configuration based on the environment type.
///
/// # Arguments
/// * `dbg` - A boolean indicating whether debug mode is enabled.
/// * `env_type` - An `EnvironmentType` enum reference representing the environment type.
///
/// # Returns
/// A `PostgresDBConfig` struct containing the Postgres database configuration.
///
pub(super) fn get_postgres_config(dbg: bool, env_type: &EnvironmentType) -> PostgresDBConfig {
    if dbg {
        println!("[CfgManager]: get_postgres_config");
    }

    if env_type == &EnvironmentType::CLUSTER {
        // Env variables for the cluster are defined in:
        // delivery/postgres/cluster.yaml
        let pg_user = get_value_from_env("PG_USER");
        let pg_password = get_value_from_env("PG_PASSWORD");
        let pg_database = get_value_from_env("PG_DATABASE");
        //
        postgres::get_cluster_db_config(pg_user, pg_password, pg_database)
    } else {
        postgres::get_postgres_config(env_type)
    }
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
pub(super) fn get_value_from_env(key: &str) -> String {
    match std::env::var(key) {
        Ok(cluster_dns_server) => cluster_dns_server,
        Err(e) => {
            panic!(
                "{} {}",
                format!("Failed to read {key} environment variable. Ensure {key} is set:"),
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
pub(crate) fn get_svc_env_config(
    dbg: bool,
    service_id: ServiceID,
    service_config: &ServiceConfig,
) -> SvcEnvConfig {
    if dbg {
        println!("[CfgManager]: build_internal_dns_resolver");
    }

    let binding = service_config.service_endpoint();
    let endpoint = binding.host_endpoint();
    let metrics_config = service_config.metrics_endpoint();
    let local_host = "0.0.0.0".to_string();
    let cluster_host = endpoint.host_uri().to_string();
    let ci_host = "127.0.0.1".to_string();
    let docker_host = "0.0.0.0".to_string();
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
