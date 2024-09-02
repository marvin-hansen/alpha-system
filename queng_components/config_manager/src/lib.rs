use std::collections::HashMap;

use common_config::prelude::{ServiceConfig, ServiceID, SvcEnvConfig};
use common_database::prelude::{ClickHouseConfig, PostgresDBConfig};
use common_env::prelude::EnvironmentType;
use common_exchange::prelude::ExchangeID;
use ctx_manager::CtxManager;
use db_specs::{clickhouse_db, postgres_db};
use dns_manager::DnsManager;
use exchange_specs::prelude;
use exchange_specs::prelude::{
    get_all_exchanges, get_all_exchanges_ids_names, get_exchange_symbol_tables,
};

use crate::utils::get_svc_env_config;

mod cfg_getters;
mod cfg_services;
mod utils;

// https://stackoverflow.com/questions/20778771/what-is-the-difference-between-0-0-0-0-127-0-0-1-and-localhost
const DEFAULT_HOST: &str = "0.0.0.0";

/// Struct that holds the configuration for a specific service.
pub struct CfgManager<'l> {
    ctx_manager: &'l CtxManager,
    dns_manager: &'l DnsManager,
    /// Type of the environment (e.g., development, testing, production).
    env_type: EnvironmentType,
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

impl<'l> CfgManager<'l> {
    pub async fn new(
        svc: ServiceID,
        svc_config: ServiceConfig,
        ctx_manager: &'l CtxManager,
        dns_manager: &'l DnsManager,
    ) -> Self {
        //
        let env_type = ctx_manager.env_type();
        let svc_env_config = get_svc_env_config(svc, &svc_config);
        // DB Config
        let db_clickhouse_config = clickhouse_db::get_clickhouse_config(&env_type);
        let db_postgres_config = postgres_db::get_postgres_config(&env_type);

        // Move this into symbol_manager
        let default_exchange = prelude::get_default_exchange();
        let exchanges = get_all_exchanges();
        let exchanges_id_names = get_all_exchanges_ids_names();
        let exchanges_symbol_tables = get_exchange_symbol_tables();

        Self {
            ctx_manager,
            dns_manager,
            env_type,
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
}
