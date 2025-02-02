/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::fields::DEFAULT_DNS;
use common_config::{ServiceConfig, ServiceID, SvcEnvConfig};
use common_env::EnvironmentType;
use common_platform::PlatformType;
use environment_manager::EnvironmentManager;
use hickory_resolver::TokioAsyncResolver;
use smdb_specs::smdb_service_config;

mod build_utils;
mod cfg_getters;
mod cfg_ims_data;
mod cfg_svc;
mod cfg_svc_health_check;
mod cfg_svc_metrics;
mod dns;
mod dns_resolve;
mod env;
mod fields;

/// Struct that holds the configuration for a specific service.
#[derive(Debug)]
pub struct CfgManager {
    /// Debug mode
    dbg: bool,
    /// Type of the environment (e.g., development, testing, production).
    env_type: EnvironmentType,
    /// Type of the platform (e.g., Linux, Windows, macOS).
    platform_type: PlatformType,
    /// Resolver for the internal (cluster) DNS server.
    internal_dns_resolver: TokioAsyncResolver,
    internal_dns_server: String,
    /// Resolver for the external (Internet) DNS server.
    external_dns_resolver: TokioAsyncResolver,
    external_dns_server: String,
    /// ID of the service.
    svc: ServiceID,
    /// Service configuration
    svc_config: ServiceConfig,
    /// Service environment configuration for each service
    svc_env_config: SvcEnvConfig,
}

impl Default for CfgManager {
    /// Returns a default instance of `CfgManager`.
    ///
    /// The default service ID is `ServiceID::Default`, and the default service
    /// configuration is `smdb_service_config()`.
    ///
    /// Debug mode is disabled by default.
    fn default() -> Self {
        Self::build(false, ServiceID::Default, smdb_service_config())
    }
}

impl CfgManager {
    ///
    /// Creates a new instance of `CfgManager`.
    ///
    /// `CfgManager` automatically adapts configurations for the detected context.
    ///
    /// `CfgManager` supports the following contexts:
    ///  * LOCAL
    ///  * CI
    ///  * CLUSTER
    ///
    /// For local environment, the `CfgManager` the following environment variables must be set:
    ///  * ENV: LOCAL
    ///
    /// For a CI environment, the `CfgManager` the following environment variables must be set:
    ///  * ENV: CI
    ///
    /// For a cluster environment, the following environment variables must be set:
    ///  * ENV: CLUSTER
    ///  * `DNS_SERVER`: The Cluster's DNS server
    ///  * `PG_USER`: The Postgres user for the cluster postgres database
    ///  * `PG_PASSWORD`: The Postgres password for the cluster postgres database
    ///  * `PG_DATABASE`: The Postgres database for the cluster postgres database
    ///
    /// If any of these variables is missing, the constructor will panic with an error
    /// indicating the missing environment variable.
    ///
    /// It is recommended to set `PG_USER`, `PG_PASSWORD` and `PG_DATABASE` as cluster secrets.
    ///
    /// In case `CfgManager` cannot detect a context, it sets the default context to UNKNOWN and
    /// configures all configurations with default values. If you observe unexpected behavior, please
    /// call the `with_debug` constructor to enable debug mode to diagnose the issue.
    ///
    /// # Arguments
    ///
    /// * `svc`: The ID of the service.
    /// * `svc_config`: The configuration of the service.
    ///
    /// # Returns
    ///
    /// The constructed instance of `CfgManager`.
    ///
    pub async fn new(svc: ServiceID, svc_config: ServiceConfig) -> Self {
        Self::build(false, svc, svc_config)
    }

    ///
    /// Creates a new instance of `CfgManager` with debug mode enabled.
    /// The `with_debug` method is identical to the `new` method, except that it enables debug mode.
    ///
    /// `CfgManager` supports the following contexts:
    ///  * LOCAL
    ///  * CI
    ///  * CLUSTER
    ///
    /// For local environment, the `CfgManager` the following environment variables must be set:
    ///  * ENV: LOCAL
    ///
    /// For a CI environment, the `CfgManager` the following environment variables must be set:
    ///  * ENV: CI
    ///
    /// For a cluster environment, the following environment variables must be set:
    ///  * ENV: CLUSTER
    ///  * `DNS_SERVER`: The Cluster's DNS server
    ///  * `PG_USER`: The Postgres user for the cluster postgres database
    ///  * `PG_PASSWORD`: The Postgres password for the cluster postgres database
    ///  * `PG_DATABASE`: The Postgres database for the cluster postgres database
    ///
    /// If any of these variables is missing, the constructor will panic with an error
    /// indicating the missing environment variable.
    ///
    /// It is recommended to set `PG_USER`, `PG_PASSWORD` and `PG_DATABASE` as cluster secrets.
    ///
    /// # Arguments
    ///
    /// * `svc`: The ID of the service.
    /// * `svc_config`: The configuration of the service.
    ///
    /// # Returns
    ///
    /// The constructed instance of `CfgManager`.
    ///
    pub async fn with_debug(svc: ServiceID, svc_config: ServiceConfig) -> Self {
        Self::build(true, svc, svc_config)
    }

    /// Returns a default instance with debug mode enabled.
    ///
    /// The default service ID is `ServiceID::Default`.
    ///
    /// The default service configuration is `smdb_service_config()`.
    ///
    /// `CfgManager` supports the following contexts:
    ///  * LOCAL
    ///  * CI
    ///  * CLUSTER
    ///
    /// For local environment, the `CfgManager` the following environment variables must be set:
    ///  * ENV: LOCAL
    ///
    /// For a CI environment, the `CfgManager` the following environment variables must be set:
    ///  * ENV: CI
    ///
    /// For a cluster environment, the following environment variables must be set:
    ///  * ENV: CLUSTER
    ///  * `DNS_SERVER`: The Cluster's DNS server
    ///  * `PG_USER`: The Postgres user for the cluster postgres database
    ///  * `PG_PASSWORD`: The Postgres password for the cluster postgres database
    ///  * `PG_DATABASE`: The Postgres database for the cluster postgres database
    ///
    /// If any of these variables is missing, the constructor will panic with an error
    /// indicating the missing environment variable.
    ///
    /// It is recommended to set `PG_USER`, `PG_PASSWORD` and `PG_DATABASE` as cluster secrets.
    ///
    #[must_use]
    pub fn default_with_debug() -> Self {
        Self::build(true, ServiceID::Default, smdb_service_config())
    }

    ///
    /// Builds a new instance by constructing various configurations
    /// including environment type, DNS servers, and database configurations.
    ///
    /// Returns the constructed instance.
    ///
    #[must_use]
    pub fn build(dbg: bool, svc: ServiceID, svc_config: ServiceConfig) -> Self {
        let env_manager = if dbg {
            println!("[CfgManager]: Debug mode enabled");
            EnvironmentManager::with_debug()
        } else {
            EnvironmentManager::new()
        };
        //
        let env_type = env_manager.env_type();
        let svc_env_config = build_utils::get_svc_env_config(dbg, svc, &svc_config);
        let platform_type = env_manager.platform_type();

        // Build the cluster internal DNS server
        let internal_dns_server = build_utils::get_internal_dns_server(dbg, &env_type);
        let internal_dns_resolver =
            build_utils::build_internal_dns_resolver(dbg, &internal_dns_server);

        // Build the external (Cloudflare) DNS address resolver to resolve hosts on the open internet
        let external_dns_server = format!("{}{}", DEFAULT_DNS, ":53");
        let external_dns_resolver = build_utils::build_external_dns_resolver(dbg);

        Self {
            dbg,
            env_type,
            platform_type,
            internal_dns_resolver,
            internal_dns_server,
            external_dns_resolver,
            external_dns_server,
            svc,
            svc_config,
            svc_env_config,
        }
    }
}

impl CfgManager {
    pub fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[CfgManager]: {msg}");
        }
    }
}
