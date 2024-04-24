use crate::env_util::ci::clickhouse;
use crate::env_util::config::clickhouse_container_config::clickhouse_container_config;
use crate::prelude::{EnvUtil, EnvironmentError};
use std::thread::sleep;
use std::time::Duration;

impl EnvUtil {
    pub async fn setup_prod(&mut self) -> Result<(), EnvironmentError> {
        // Get clickhouse container config
        let container_config = clickhouse_container_config();
        let clickhouse_container_name = container_config.name().to_string();
        let clickhouse_container_port = container_config.additional_ports()[0];

        // get clickhouse client
        let client = self.get_clickhouse_client(&container_config).await;

        // Init, reset, or reuse clickhouse DB
        clickhouse::setup::configure_reset_or_reuse_clickhouse_db(&client, &container_config)
            .await
            .expect("[TestEnv:CI]: Failed to init or reuse clickhouse db");

        // Give the container some extra time to complete initialization.
        // Otherwise, you may get a connection refused error. Adjust the time if needed.
        sleep(Duration::from_millis(100));

        self.set_clickhouse_container_name(clickhouse_container_name);
        self.set_clickhouse_container_port(clickhouse_container_port);

        Ok(())
    }

    pub async fn teardown_prod(&self) -> Result<(), EnvironmentError> {
        // get clickhouse client
        let container_config = clickhouse_container_config();
        let client = self.get_clickhouse_client(&container_config).await;

        // Remove all databases
        clickhouse::teardown::drop_databases(&client)
            .await
            .expect("Failed to drop databases");

        Ok(())
    }
}
