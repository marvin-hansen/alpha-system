use common_config::prelude::{ServiceConfig, ServiceID, SvcEnvConfig};
use common_database::prelude::{ClickHouseConfig, PostgresDBConfig};
use common_env::prelude::EnvironmentType;
use common_exchange::prelude::ExchangeID;
use exchange_specs::prelude;
use exchange_specs::prelude::{
    get_all_exchanges, get_all_exchanges_ids_names, get_exchange_symbol_tables,
};
use hickory_resolver::TokioAsyncResolver;
use smdb_specs::smdb_service_config;
use std::collections::HashMap;

mod build_utils;
mod cfg_getters;
mod cfg_svc;
mod cfg_svc_health_check;
mod cfg_svc_metrics;
mod dns;
mod dns_resolve;
mod env;

// https://stackoverflow.com/questions/20778771/what-is-the-difference-between-0-0-0-0-127-0-0-1-and-localhost
const DEFAULT_HOST: &str = "0.0.0.0";

// https://www.dnsperf.com/#!dns-resolvers
pub(crate) const DEFAULT_DNS: &str = "1.1.1.1";

/// Struct that holds the configuration for a specific service.
pub struct CfgManager {
    dbg: bool,
    /// Type of the environment (e.g., development, testing, production).
    env_type: EnvironmentType,
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
    /// ClickHouse configuration.
    db_clickhouse_config: ClickHouseConfig,
    /// Postgres configuration.
    db_postgres_config: PostgresDBConfig,
    /// Default exchange
    default_exchange: ExchangeID,
    /// Vector of all supported exchanges.
    exchanges: Vec<ExchangeID>,
    /// Maps exchange IDs to their names. Used to configure Symbol Manager
    exchanges_id_names: Vec<(u16, String)>,
    /// Maps exchange IDs to their symbol table. Used to configure Query Manager
    exchanges_symbol_tables: HashMap<ExchangeID, String>,
}

impl Default for CfgManager {
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
    ///  * DNS_SERVER: The Cluster's DNS server
    ///  * PG_USER: The Postgres user for the cluster postgres database
    ///  * PG_PASSWORD: The Postgres password for the cluster postgres database
    ///  * PG_DATABASE: The Postgres database for the cluster postgres database
    ///
    /// If any of these variables is missing, the constructor will panic with an error
    /// indicating the missing environment variable.
    ///
    /// It is recommended to set PG_USER, PG_PASSWORD and PG_DATABASE as cluster secrets.
    ///
    /// In case `CfgManager` cannot detect a context, it sets the default context to UNKNOWN and
    /// configures all configurations with default values. If you observe unexpected behavior, please
    /// call the with_debug constructor to enable debug mode to diagnose the issue.
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
    ///  * DNS_SERVER: The Cluster's DNS server
    ///  * PG_USER: The Postgres user for the cluster postgres database
    ///  * PG_PASSWORD: The Postgres password for the cluster postgres database
    ///  * PG_DATABASE: The Postgres database for the cluster postgres database
    ///
    /// If any of these variables is missing, the constructor will panic with an error
    /// indicating the missing environment variable.
    ///
    /// It is recommended to set PG_USER, PG_PASSWORD and PG_DATABASE as cluster secrets.
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
    ///  * DNS_SERVER: The Cluster's DNS server
    ///  * PG_USER: The Postgres user for the cluster postgres database
    ///  * PG_PASSWORD: The Postgres password for the cluster postgres database
    ///  * PG_DATABASE: The Postgres database for the cluster postgres database
    ///
    /// If any of these variables is missing, the constructor will panic with an error
    /// indicating the missing environment variable.
    ///
    /// It is recommended to set PG_USER, PG_PASSWORD and PG_DATABASE as cluster secrets.
    ///
    pub fn default_with_debug() -> Self {
        Self::build(true, ServiceID::Default, smdb_service_config())
    }

    ///
    /// Builds a new instance by constructing various configurations
    /// including environment type, DNS servers, and database configurations.
    ///
    /// Returns the constructed instance.
    ///
    pub fn build(dbg: bool, svc: ServiceID, svc_config: ServiceConfig) -> Self {
        //
        let env_type = Self::detect_env_type(dbg);
        let svc_env_config = build_utils::get_svc_env_config(dbg, svc, &svc_config);

        // Build the cluster internal DNS server
        let internal_dns_server = build_utils::build_internal_dns_server(dbg, &env_type);
        let internal_dns_resolver =
            build_utils::build_internal_dns_resolver(dbg, &internal_dns_server);

        // Build the external (Cloudflare) DNS address resolver to resolve hosts on the open internet
        let external_dns_server = format!("{}{}", DEFAULT_DNS, ":53");
        let external_dns_resolver = build_utils::build_external_dns_resolver(dbg);

        // DB Config
        let db_clickhouse_config = build_utils::get_clickhouse_config(dbg, &env_type);
        let db_postgres_config = build_utils::get_postgres_config(dbg, &env_type);

        // Remove this after adding MDDB service
        let default_exchange = prelude::get_default_exchange();
        let exchanges = get_all_exchanges();
        let exchanges_id_names = get_all_exchanges_ids_names();
        let exchanges_symbol_tables = get_exchange_symbol_tables();

        Self {
            dbg,
            env_type,
            internal_dns_resolver,
            internal_dns_server,
            external_dns_resolver,
            external_dns_server,
            svc,
            svc_config,
            svc_env_config,
            db_clickhouse_config,
            db_postgres_config,
            // Remove this after adding MDDB service
            default_exchange,
            exchanges,
            exchanges_id_names,
            exchanges_symbol_tables,
        }
    }

    ///
    /// Detects the environment type based on the value of the "ENV" environment variable.
    ///
    /// * If the variable is set to "CI", returns EnvironmentType::CI.
    /// * If set to "CLUSTER", returns EnvironmentType::CLUSTER.
    /// * If set to "LOCAL", returns EnvironmentType::LOCAL.
    /// * If set to "UNKNOWN" or any other value, returns EnvironmentType::UNKNOWN.
    ///
    /// Prints debug messages if the 'dbg' parameter is true.
    ///
    /// Panics if unable to read the "ENV" environment variable.
    ///
    /// Returns the detected EnvironmentType.
    ///
    pub fn detect_env_type(dbg: bool) -> EnvironmentType {
        if dbg {
            println!("[CfgManager]: Debug mode enabled");
        }

        // Check if the environment variable is set.
        // If so, return local environment as the file only exists locally.
        // If not, return UnknownEnv.
        // On Mac OS, each shell environment variables is sanitized (erased) by default for security reasonsm
        let env_type = match std::env::var("ENV") {
            Ok(val) => match val.as_str() {
                "CI" => EnvironmentType::CI,
                "CLUSTER" => EnvironmentType::CLUSTER,
                "LOCAL" => EnvironmentType::LOCAL,
                "UNKNOWN" => EnvironmentType::UNKNOWN,
                _ => EnvironmentType::UNKNOWN,
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                panic!("[CfgManager]: Failed to read ENV environment variable. Ensure ENV is set");
            }
        };

        if dbg {
            println!("[CfgManager]: Detected environment type: {:?}", &env_type);
        }

        env_type
    }
}

impl CfgManager {
    pub fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[CfgManager]: {}", msg);
        }
    }
}
