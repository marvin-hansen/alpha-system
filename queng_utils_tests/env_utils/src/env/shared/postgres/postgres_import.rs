use crate::prelude::EnvironmentError;
use crate::EnvUtil;
use portfolio_specs::prelude::get_all_portfolio_specs;
use postgres_utils::PostgresUtil;
use service_specs_all::prelude::get_all_service_specs;

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

        match pg_utils.import_service_collection(&services).await {
            Ok(_) => {}
            Err(err) => {
                return Err(EnvironmentError::new(format!(
                    "[import_pg_service_data]: Failed to import services due error: {}",
                    err
                )))
            }
        }

        Ok(())
    }
    async fn import_pg_portfolio_data(
        &self,
        pg_utils: &PostgresUtil,
    ) -> Result<(), EnvironmentError> {
        self.dbg_print("import_pg_portfolio_data");

        let portfolios = get_all_portfolio_specs();

        match pg_utils.import_portfolio_collection(&portfolios).await {
            Ok(_) => {}
            Err(err) => {
                return Err(EnvironmentError::new(format!(
                    "[import_pg_portfolio_data]: Failed to import portfolios due error: {}",
                    err
                )))
            }
        }

        Ok(())
    }
}
