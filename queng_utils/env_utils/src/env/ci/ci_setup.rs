use specs_utils::prelude::clickhouse_container_specs;

use crate::prelude::{EnvUtil, EnvironmentError};

impl EnvUtil {
    /// Sets up the environment for Continuous Integration (CI) testing.
    ///
    /// This function performs the following steps:
    ///
    /// 1. Sets the data sample size to 10% of the available data.
    /// 2. Gets or reuses all containers required for testing.
    /// 3. Gets the configuration for the ClickHouse container.
    /// 4. Gets the ClickHouse utilities.
    /// 5. Gets the Kaiko utilities.
    /// 6. Configures the ClickHouse database.
    /// 7. Verifies the ClickHouse database.
    ///
    /// # Errors
    ///
    /// - `EnvironmentError` if any step fails.
    ///
    pub async fn setup_ci(&mut self) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[setup_ci]: Check if CI environment already configured");
        if self.ci_env_configured {
            self.dbg_print("[setup_ci]: CI environment already configured.");
            return Ok(());
        }

        self.dbg_print("[setup_ci]: Set data sample size to 10%");
        let sample_size = None; // Some(0);

        self.dbg_print("[setup_ci]: Get or reuse all containers");
        self.setup_containers()
            .await
            .expect("[setup_ci]: Failed to setup containers");

        self.dbg_print("[setup_ci]: Get clickhouse container config");
        let clickhouse_container_config = clickhouse_container_specs();

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

        Ok(())
    }
}
