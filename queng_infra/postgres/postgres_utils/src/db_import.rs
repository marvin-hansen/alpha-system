use crate::prelude::PostgresUtilError;
use crate::PostgresUtil;
use common_config::prelude::ServiceConfig;
use common_exchange::prelude::PortfolioConfig;

use pg_smdb::prelude::service;
impl PostgresUtil {
    /// Imports a collection of service configurations into the SMDB database.
    ///
    /// # Arguments
    ///
    /// * `services` - A slice of `ServiceConfig` objects.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if the insertion operation fails.
    ///
    pub async fn import_service_collection(
        &self,
        services: &[ServiceConfig],
    ) -> Result<(), PostgresUtilError> {
        let conn = &mut self.pool.get().unwrap();

        self.dbg_print("[import_service_data]: Import services into SMDB DB schema");
        match service::Service::insert_service_collection(conn, services) {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }

    /// Imports a collection of portfolio configurations into the CMDB database.
    ///
    /// # Arguments
    ///
    /// * `portfolios` - A slice of `PortfolioConfig` objects.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if the insertion operation fails.
    ///
    pub async fn import_portfolio_collection(
        &self,
        _portfolios: &[PortfolioConfig],
    ) -> Result<(), PostgresUtilError> {
        // Implement
        Ok(())
    }
}
