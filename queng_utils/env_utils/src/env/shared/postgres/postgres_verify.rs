use postgres_utils::PostgresUtil;
use specs_utils::prelude::{get_all_portfolio_specs, get_all_service_specs};

use crate::prelude::{EnvUtil, EnvironmentError};

impl EnvUtil {
    /// Asynchronously verifies if the Postgres database is set up correctly.
    ///
    /// This method verifies if the Postgres database is set up correctly by performing the following steps:
    /// - Retrieve the `PostgresUtil` object.
    /// - Verify if all databases are created.
    /// - Verify if all data are imported.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that indicates if the Postgres database is set up correctly.
    /// If successful, it returns `Ok(true)`.
    /// If an error occurs, it returns `Err(EnvironmentError)`.
    ///
    pub(crate) async fn verify_postgres_db(&self) -> Result<bool, EnvironmentError> {
        self.dbg_print("verify_postgres_db");

        self.dbg_print("[verify_postgres_db]; Get Postgres util");
        let pg_util = self
            .get_new_postgres_util()
            .await
            .expect("[verify_postgres_db]: Failed to get PostgresUtil");

        let setup = pg_util
            .verify_all_db()
            .await
            .expect("Failed to verify postgres DB");
        if !setup {
            return Err(EnvironmentError::from("Failed to verify postgres DB"));
        }

        let imported = self
            .verify_postgres_data_imported(&pg_util)
            .await
            .expect("Failed to verify postgres data imported");
        if !imported {
            return Err(EnvironmentError::from(
                "Failed to verify postgres data imported",
            ));
        }

        Ok(true)
    }

    /// Verifies if all data has been imported into the Postgres database.
    ///
    /// This method performs the following steps:
    ///
    /// 1. Verifies if all service data has been imported.
    /// 2. Verifies if all portfolio data has been imported.
    ///
    /// # Arguments
    ///
    /// * `pg_util` - A reference to a `PostgresUtil` object.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that indicates whether all data has been imported into the Postgres database.
    /// If successful, it returns `Ok(bool)` indicating whether all data has been imported.
    /// If an error occurs, it returns `Err(EnvironmentError)`.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `EnvironmentError`.
    ///
    pub(crate) async fn verify_postgres_data_imported(
        &self,
        pg_util: &PostgresUtil,
    ) -> Result<bool, EnvironmentError> {
        self.dbg_print("verify_postgres_data_imported");

        self.dbg_print("[verify_postgres_data_imported]: verify_service_data_imported");
        let service_imported = self
            .verify_service_data_imported(pg_util)
            .await
            .expect("Failed to verify service data imported");

        self.dbg_print(&format!(
            "[verify_postgres_data_imported]: service_imported: {}",
            service_imported
        ));

        self.dbg_print("[verify_postgres_data_imported]: verify_portfolio_data_imported");
        let portfolio_imported = self
            .verify_portfolio_data_imported(pg_util)
            .await
            .expect("Failed to verify portfolio data imported");

        self.dbg_print(&format!(
            "[verify_postgres_data_imported]: portfolio_imported: {}",
            portfolio_imported
        ));
        let all_imported = service_imported && portfolio_imported;

        Ok(all_imported)
    }

    pub(crate) async fn verify_service_data_imported(
        &self,
        pg_util: &PostgresUtil,
    ) -> Result<bool, EnvironmentError> {
        self.dbg_print("verify_service_data_imported");

        let nr_services = get_all_service_specs().len() as u64;

        let nr_db_services =
            pg_util.specs.count_services().await.expect(
                "[verify_service_data_imported]: Failed to get nr services from metadata DB",
            );

        let services_imported = nr_services == nr_db_services;
        self.dbg_print(
            format!(
                "[verify_service_data_imported]: All services imported: {}",
                services_imported
            )
            .as_str(),
        );

        Ok(services_imported)
    }

    async fn verify_portfolio_data_imported(
        &self,
        pg_util: &PostgresUtil,
    ) -> Result<bool, EnvironmentError> {
        self.dbg_print("verify_portfolio_data_imported");

        let nr_portfolios = get_all_portfolio_specs().len() as u64;

        let nr_db_portfolios = pg_util.specs.count_portfolios().await.expect(
            "[verify_portfolio_data_imported]: Failed to get nr portfolios from metadata DB",
        );

        let portfolios_imported = nr_portfolios == nr_db_portfolios;
        self.dbg_print(
            format!(
                "[verify_portfolio_data_imported]: All portfolios imported: {}",
                portfolios_imported
            )
            .as_str(),
        );

        Ok(portfolios_imported)
    }
}
