use crate::utils::get_svc_env_config;
use common_config::prelude::{ServiceConfig, ServiceID, SvcEnvConfig};
use common_database::prelude::{ClickHouseConfig, PostgresDBConfig};
use common_env::prelude::EnvironmentType;
use common_exchange::prelude::ExchangeID;
use db_specs::{clickhouse_db, postgres_db};
use exchange_specs::prelude;
use exchange_specs::prelude::{
    get_all_exchanges, get_all_exchanges_ids_names, get_exchange_symbol_tables,
};
use hickory_resolver::config::*;
use hickory_resolver::TokioAsyncResolver;
use smdb_specs::smdb_service_config;
use std::collections::HashMap;
use std::net::SocketAddr;

mod cfg_getters;
mod cfg_services;
mod dns;
mod dns_resolve;
mod env;
mod utils;

// https://stackoverflow.com/questions/20778771/what-is-the-difference-between-0-0-0-0-127-0-0-1-and-localhost
const DEFAULT_HOST: &str = "0.0.0.0";

// https://www.dnsperf.com/#!dns-resolvers
const DEFAULT_DNS: &str = "1.1.1.1";

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

    pub fn build(dbg: bool, svc: ServiceID, svc_config: ServiceConfig) -> Self {
        //
        let env_type = Self::detect_env_type(dbg);
        let svc_env_config = get_svc_env_config(svc, &svc_config);
        // DB Config
        let db_clickhouse_config = clickhouse_db::get_clickhouse_config(&env_type);
        let db_postgres_config = postgres_db::get_postgres_config(&env_type);

        let internal_dns_server = Self::build_internal_dns_server(&env_type);
        let internal_dns_resolver = Self::build_internal_dns_resolver(&internal_dns_server);

        // Build the external (Cloudflare) DNS address resolver to resolve hosts on the open internet
        let external_dns_server = format!("{}{}", DEFAULT_DNS, ":53");
        let external_dns_resolver = Self::build_external_dns_resolver();

        // Move this into symbol_manager
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
            default_exchange,
            exchanges,
            exchanges_id_names,
            exchanges_symbol_tables,
        }
    }

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

    fn build_external_dns_resolver() -> TokioAsyncResolver {
        let external_resolver_config = ResolverConfig::cloudflare();
        TokioAsyncResolver::tokio(external_resolver_config, ResolverOpts::default())
    }

    fn build_internal_dns_resolver(address: &str) -> TokioAsyncResolver {
        let socket_addr: SocketAddr = match address.parse() {
            Ok(addr) => addr,
            Err(e) => panic!("Failed to parse DNS SERVER address: {}", e),
        };

        let name_server = NameServerConfig::new(socket_addr, Protocol::Udp);

        let mut config = ResolverConfig::new();

        config.add_name_server(name_server);

        TokioAsyncResolver::tokio(config.clone(), ResolverOpts::default())
    }

    fn build_internal_dns_server(env_type: &EnvironmentType) -> String {
        // Find the internal DNS server based on the env context
        let internal_dns_host = match env_type {
            EnvironmentType::LOCAL => DEFAULT_DNS.to_owned(),
            EnvironmentType::CI => DEFAULT_DNS.to_owned(),
            EnvironmentType::CLUSTER => Self::get_cluster_dns(),
            EnvironmentType::UNKNOWN => DEFAULT_DNS.to_owned(),
        };

        // Build the internal DNS resolver to resolve hosts within the system network
        format!("{}{}", internal_dns_host, ":53")
    }

    fn get_cluster_dns() -> String {
        match std::env::var("DNS_SERVER") {
            Ok(cluster_dns_server) => cluster_dns_server,
            Err(e) => {
                panic!(
                    "Failed to read DNS_SERVER environment variable. Ensure DNS_SERVER is set in deployment.yaml:{}",
                    e
                );
            }
        }
    }
}

impl CfgManager {
    pub fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[CtxManager]: {}", msg);
        }
    }
}
