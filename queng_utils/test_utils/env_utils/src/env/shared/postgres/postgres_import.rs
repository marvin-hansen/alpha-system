use postgres_utils::PostgresUtil;
use specs_utils::prelude::{get_all_portfolio_specs, get_all_service_specs};

use crate::prelude::EnvironmentError;
use crate::EnvUtil;

impl EnvUtil {
    /// Asynchronously imports all data into all Postgres databases.
    ///
    /// This method performs the following steps:
    ///
    /// 1. Verifies that all databases have been created.
    /// 2. Imports all service data into the service database.
    /// 3. Imports all portfolio data into the portfolio database.
    ///
    /// # Arguments
    ///
    /// * `pg_utils` - A reference to the `PostgresUtil` object.
    ///
    /// # Errors
    ///
    /// T
    pub(crate) async fn import_all_pg_data(
        &self,
        pg_utils: &PostgresUtil,
    ) -> Result<(), EnvironmentError> {
        self.dbg_print("import_all_pg_data");

        self.dbg_print("[import_all_pg_data]: Import service data into Postgres");
        self.import_pg_service_data(pg_utils)
            .await
            .expect("[import_all_pg_data]: Failed to import service data into Postgres");

        self.dbg_print("[import_all_pg_data]: Import portfolio data into Postgres");
        self.import_pg_portfolio_data(pg_utils)
            .await
            .expect("[import_all_pg_data]: Failed to import portfolio data into Postgres");

        Ok(())
    }

    async fn import_pg_service_data(
        &self,
        pg_utils: &PostgresUtil,
    ) -> Result<(), EnvironmentError> {
        self.dbg_print("import_pg_service_data");

        let services = get_all_service_specs();

        for service in services {
            match pg_utils.specs.insert_service(&service).await {
                Ok(_) => self.dbg_print(&format!(
                    "[import_pg_service_data]: Imported service: {}",
                    service.name()
                )),
                Err(err) => {
                    return Err(EnvironmentError::new(format!(
                        "[import_pg_service_data]: Failed to import service: {} due error: {}",
                        service.name(),
                        err
                    )))
                }
            };
        }

        Ok(())
    }
    async fn import_pg_portfolio_data(
        &self,
        pg_utils: &PostgresUtil,
    ) -> Result<(), EnvironmentError> {
        self.dbg_print("import_pg_portfolio_data");

        let portfolios = get_all_portfolio_specs();

        for portfolio in portfolios {
            match pg_utils.specs.insert_portfolio(&portfolio).await {
                Ok(_) => self.dbg_print(&format!(
                    "[import_pg_portfolio_data]: Imported portfolio: {}",
                    portfolio.portfolio_id()
                )),
                Err(err) => {
                    return Err(EnvironmentError::new(format!(
                        "[import_pg_portfolio_data]: Failed to import portfolio: {} due error: {}",
                        portfolio.portfolio_id(),
                        err
                    )))
                }
            };
        }

        Ok(())
    }
}
