use container_specs_clickhouse::clickhouse_container_config;

use crate::prelude::{EnvUtil, EnvironmentError};

impl EnvUtil {
    /// Sets up the environment for Continuous Integration (CI) testing.
    ///
    /// This function performs the following steps:
    ///
    /// 1. Sets the data sample size to 10% of the available data.
    /// 2. Gets or reuses all containers required for testing.
    /// 3. Gets the configuration for the ClickHouse container.
    /// 4. Configures the ClickHouse database.
    /// 5. Verifies the ClickHouse database.
    /// 6. Configures the PostgreSQL database.
    /// 7. Verifies the PostgreSQL database.
    ///
    /// # Errors
    ///
    /// Returns an `EnvironmentError` if any of the following steps fail:
    ///
    /// - Setting up any of the required containers.
    /// - Configuring or verifying the ClickHouse database.
    /// - Configuring or verifying the PostgreSQL database.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    ///
    /// - Retrieving the ClickHouse container configuration fails.
    /// - Setting up the ClickHouse database fails.
    /// - Verifying the ClickHouse database fails.
    /// - Setting up the PostgreSQL database fails.
    /// - Verifying the PostgreSQL database fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use env_utils::prelude::{EnvUtil, EnvironmentError};
    /// # async fn example() -> Result<(), EnvironmentError> {
    /// let mut env_util = EnvUtil::new().await?;
    /// env_util.setup_ci().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the environment is successfully configured,
    /// or an `Err` variant of `EnvironmentError` if an error occurs during the configuration process.
    ///
    pub async fn setup_ci(&self) -> Result<(), EnvironmentError> {
        self.dbg_print("[setup_ci]: Setup CI environment");

        self.dbg_print("[setup_ci]: Check if CI environment already configured");
        if self.ci_env_configured() {
            self.dbg_print("[setup_ci]: CI environment already configured.");
            return Ok(());
        }

        self.dbg_print("[setup_ci]: Set data sample size to 10%");
        let sample_size = None; // Some(0);

        self.dbg_print("[setup_ci]: Get or reuse all containers");
        self.setup_all_containers()
            .await
            .expect("[setup_ci]: Failed to setup containers");

        self.dbg_print("[setup_ci]: Get clickhouse container config");
        let clickhouse_container_config = clickhouse_container_config();

        self.dbg_print("[setup_ci]: Configure clickhouse DB");
        self.setup_clickhouse(&clickhouse_container_config, sample_size)
            .await
            .expect("[setup_ci]: Failed to configure clickhouse DB");

        let ch_configured = self
            .verify_clickhouse_db()
            .await
            .expect("[setup_ci]: Failed to verify clickhouse DB");

        if !ch_configured {
            return Err(EnvironmentError::from(
                "clickhouse not correctly configured",
            ));
        }

        self.dbg_print("[setup_ci]: Configure Postgres DB");
        self.setup_postgres()
            .await
            .expect("[setup_ci]: Failed to configure Postgres DB");

        self.dbg_print("[setup_ci]: Verify Postgres DB");
        let pg_configured = self
            .verify_postgres_db()
            .await
            .expect("[setup_ci]: Failed to verify Postgres DB");

        if !pg_configured {
            return Err(EnvironmentError::from("Postgres not correctly configured"));
        }

        Ok(())
    }
}
