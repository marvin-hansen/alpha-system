// Unsafe code must be explicitly enabled to use it.
#[deny(unsafe_code)]
//
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
    pub async fn new(svc: ServiceID, svc_config: ServiceConfig) -> Self {
        Self::build(false, svc, svc_config)
    }

    pub async fn with_debug(svc: ServiceID, svc_config: ServiceConfig) -> Self {
        Self::build(true, svc, svc_config)
    }

    pub fn default_with_debug() -> Self {
        Self::build(true, ServiceID::Default, smdb_service_config())
    }

    ///
    /// Builds a new instance by constructing various configurations
    /// including environment type, DNS servers, and database configurations.
    ///
    /// Returns the constructed instance.
    ///
    fn build(dbg: bool, svc: ServiceID, svc_config: ServiceConfig) -> Self {
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
