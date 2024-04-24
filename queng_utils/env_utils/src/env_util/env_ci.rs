use crate::env_util::ci::clickhouse;
use crate::env_util::config::clickhouse_container_config::clickhouse_container_config;
use crate::prelude::{EnvUtil, EnvironmentError};
use std::thread::sleep;
use std::time::Duration;

impl EnvUtil {
    /// Create a new Continuous Integration (CI) `Environment`
    pub async fn setup_ci(&mut self) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("Get docker util");
        let mut docker_util = self
            .get_docker_util()
            .expect("[TestEnv:CI]: Failed to get docker util");

        self.dbg_print("Get clickhouse container config");
        let container_config = clickhouse_container_config();

        self.dbg_print("Get or reuse clickhouse container");
        let (clickhouse_container_name, clickhouse_container_port) = docker_util
            .get_or_start_container_config(&container_config)
            .expect("[TestEnv:CI]: Failed to get or reuse clickhouse container");

        self.dbg_print("Get clickhouse client");
        let client = self.get_clickhouse_client(&container_config).await;

        self.dbg_print("Init, reset, or reuse clickhouse Database");
        clickhouse::setup::configure_reset_or_reuse_clickhouse_db(&client, &container_config)
            .await
            .expect("[TestEnv:CI]: Failed to init or reuse clickhouse db");

        // Give the container some extra time to complete initialization.
        // Otherwise, you may get a connection refused error. Adjust the time if needed.
        sleep(Duration::from_millis(100));

        self.dbg_print("Set container name and port");
        self.set_clickhouse_container_name(clickhouse_container_name);
        self.set_clickhouse_container_port(clickhouse_container_port);

        Ok(())
    }

    // teardown CI instance of test environment
    pub async fn teardown_ci(&self) -> Result<(), EnvironmentError> {
        self.dbg_print("Get clickhouse client");
        let container_config = clickhouse_container_config();
        let client = self.get_clickhouse_client(&container_config).await;

        self.dbg_print("Remove all databases");
        clickhouse::teardown::drop_databases(&client)
            .await
            .expect("Failed to drop databases");

        self.dbg_print("Get docker util");
        let mut docker_util = self
            .get_docker_util()
            .expect("[TestEnv:CI]: Failed to get docker util");

        self.dbg_print("Get container id");
        let container_id = self.clickhouse_container_name();

        self.dbg_print("Stop and remove container");
        docker_util
            .stop_container(container_id)
            .expect("[TestEnv:CI]: Failed to teardown clickhouse container");

        Ok(())
    }
}
