use common::prelude::{ClickHouseConfig, EnvironmentType, ExchangeID, ServiceID, SvcEnvConfig};
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
use exchange_specs::prelude;
use exchange_specs::prelude::{
    get_all_exchanges, get_all_exchanges_ids_names, get_exchange_symbol_tables,
};
use std::cell::RefCell;
use std::collections::HashMap;

mod cfg_getters;
mod cfg_services;
mod utils;

// https://stackoverflow.com/questions/20778771/what-is-the-difference-between-0-0-0-0-127-0-0-1-and-localhost
const DEFAULT_HOST: &str = "0.0.0.0";

/// Struct that holds the configuration for a specific service.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CfgManager<'l> {
    ctx_manager: &'l CtxManager,
    dns_manager: &'l DnsManager,
    /// ID of the service.
    svc: ServiceID,
    /// ClickHouse configuration.
    clickhouse_config: ClickHouseConfig,
    /// Type of the environment (e.g., development, testing, production).
    env_type: EnvironmentType,
    /// Default exchange
    default_exchange: ExchangeID,
    /// Vector of all supported exchanges.
    exchanges: Vec<ExchangeID>,
    /// Maps exchange IDs to their names. Used to configure Symbol Manager
    exchanges_id_names: Vec<(u16, String)>,
    /// Maps exchange IDs to their symbol table. Used to configure Query Manager
    exchanges_symbol_tables: HashMap<ExchangeID, String>,
    //
    cmdb_env: RefCell<Option<SvcEnvConfig>>,
    smdb_env: RefCell<Option<SvcEnvConfig>>,
    symdb_env: RefCell<Option<SvcEnvConfig>>,
    dbgw_env: RefCell<Option<SvcEnvConfig>>,
    qdgw_env: RefCell<Option<SvcEnvConfig>>,
    vex_env: RefCell<Option<SvcEnvConfig>>,
}

impl<'l> CfgManager<'l> {
    pub fn new(svc: ServiceID, ctx_manager: &'l CtxManager, dns_manager: &'l DnsManager) -> Self {
        let env_type = ctx_manager.env_type();
        // Load specifications
        let clickhouse_config = utils::get_db_config(&env_type);
        let default_exchange = prelude::get_default_exchange();
        let exchanges = get_all_exchanges();
        let exchanges_id_names = get_all_exchanges_ids_names();
        let exchanges_symbol_tables = get_exchange_symbol_tables();

        Self {
            ctx_manager,
            dns_manager,
            svc,
            clickhouse_config,
            env_type,
            default_exchange,
            exchanges,
            exchanges_id_names,
            exchanges_symbol_tables,
            cmdb_env: RefCell::new(None),
            smdb_env: RefCell::new(None),
            symdb_env: RefCell::new(None),
            dbgw_env: RefCell::new(None),
            qdgw_env: RefCell::new(None),
            vex_env: RefCell::new(None),
        }
    }
}
