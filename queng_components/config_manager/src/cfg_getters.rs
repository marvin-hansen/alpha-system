use crate::CfgManager;
use common::prelude::ExchangeID;
use common_config::prelude::{
    ClickHouseConfig, EnvironmentType, ServiceConfig, ServiceID, SurrealDBConfig,
};
use common_errors::prelude::InitError;

impl<'l> CfgManager<'l> {
    pub fn get_svc_id(&self) -> ServiceID {
        self.svc
    }

    pub fn get_env_type(&self) -> EnvironmentType {
        self.env_type
    }

    pub fn get_svc_config(&self) -> ServiceConfig {
        self.svc_config.to_owned()
    }

    pub fn get_svc_host_port(&self) -> Result<(String, u16), InitError> {
        // Get the configuration of the service
        let svc_config = &self.svc_env_config;
        // Get the host and port of the service
        self.get_host(svc_config)
    }

    pub fn clickhouse_db_specs_config(&self) -> &ClickHouseConfig {
        &self.db_specs_config
    }

    pub fn clickhouse_db_metadata_config(&self) -> &ClickHouseConfig {
        &self.db_metadata_config
    }

    pub fn surreal_db_config(&self) -> &SurrealDBConfig {
        &self.db_surreal_config
    }
}

impl<'l> CfgManager<'l> {
    /// Get the default ExchangeID configured for this service.
    ///
    /// # Returns
    ///
    /// The `default_exchange` field, containing the u16 ExchangeID value
    /// set as the default for this service.
    pub fn default_exchange(&self) -> ExchangeID {
        self.default_exchange
    }

    /// Get a reference to the vector of configured ExchangeIDs.
    ///
    /// # Returns
    ///
    /// A reference to the `exchanges` field, which is a vector containing
    /// the ExchangeID values configured for this service.
    pub fn exchanges(&self) -> &Vec<ExchangeID> {
        &self.exchanges
    }

    /// Get a reference to the vector containing ExchangeID and name pairs.
    ///
    /// # Returns
    ///
    /// A reference to the `exchanges_id_names` field, which contains a vector
    /// of tuples with the first element being the ExchangeID u16 and the
    /// second element being the corresponding exchange name string.
    pub fn exchanges_id_names(&self) -> &Vec<(u16, String)> {
        &self.exchanges_id_names
    }

    /// Retrieves the symbol table name for the given exchange and symbol IDs.
    ///
    /// # Arguments
    ///
    /// * `exchange_id` - The ID of the exchange
    /// * `symbol_id` - The ID of the symbol
    ///
    /// # Returns
    ///
    /// Returns an Option of either a `String` containing the symbol table name or None.,
    ///
    pub fn get_symbol_table(&self, exchange_id: ExchangeID) -> Option<String> {
        self.exchanges_symbol_tables.get(&exchange_id).cloned()
    }
}
