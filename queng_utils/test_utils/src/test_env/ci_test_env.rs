use crate::prelude::{TestEnv, TestEnvError};
use crate::test_env::ci::clickhouse;
use crate::test_env::config::clickhouse_container_config::clickhouse_container_config;
use crate::test_env::shared;
use std::thread::sleep;
use std::time::Duration;

impl TestEnv {
    /// Create a new instance of `TestEnv` for Continuous Integration (CI).
    pub async fn setup_ci() -> Result<Self, TestEnvError> {
        // Get docker util
        let mut docker_util =
            shared::get_docker_util().expect("[TestEnv:CI]: Failed to get docker util");

        // Get clickhouse container config
        let container_config = clickhouse_container_config();

        // get clickhouse client
        let client = shared::get_clickhouse_client(&container_config).await;

        let (clickhouse_container_name, clickhouse_container_port) = docker_util
            .get_or_start_container_config(&container_config)
            .expect("[TestEnv:CI]: Failed to get or reuse clickhouse container");

        // Init, reset, or reuse clickhouse DB
        clickhouse::setup::configure_reset_or_reuse_clickhouse_db(&client, &container_config)
            .await
            .expect("[TestEnv:CI]: Failed to init or reuse clickhouse db");

        // Give the container some extra time to complete initialization.
        // Otherwise, you may get a connection refused error. Adjust the time if needed.
        sleep(Duration::from_millis(100));

        Ok(Self {
            docker_util,
            clickhouse_container_name,
            clickhouse_container_port,
        })
    }

    // teardown CI instance of test environment
    pub async fn teardown_ci(&self) -> Result<(), TestEnvError> {
        // get clickhouse client
        let container_config = clickhouse_container_config();
        let client = shared::get_clickhouse_client(&container_config).await;

        // Remove all databases
        clickhouse::teardown::drop_databases(&client)
            .await
            .expect("Failed to drop databases");

        // Get docker util
        let mut docker_util =
            shared::get_docker_util().expect("[TestEnv:CI]: Failed to get docker util");

        // Get Clickhouse container id
        let container_id = self.clickhouse_container_name();

        // Remove clickhouse container
        docker_util
            .stop_container(container_id)
            .expect("[TestEnv:CI]: Failed to teardown clickhouse container");

        Ok(())
    }
}
