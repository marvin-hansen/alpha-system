mod clickhouse;
mod metadata;
mod specs;

use crate::docker_util::DockerUtil;
use crate::prelude::{DockerError, TestEnv, TestEnvError};
use crate::test_env::config::clickhouse_container_config::clickhouse_container_config;
use std::thread::sleep;
use std::time::Duration;

impl TestEnv {
    /// Create a new instance of `TestEnv` for Continuous Integration (CI).
    pub async fn setup_ci() -> Result<Self, TestEnvError> {
        // Get docker util
        let mut docker_util = get_docker_util().expect("[TestEnv:CI]: Failed to get docker util");

        // Get or reuse clickhouse container
        let container_config = clickhouse_container_config();

        let (clickhouse_container_name, clickhouse_container_port) = docker_util
            .get_or_start_container_config(&container_config)
            .expect("[TestEnv:CI]: Failed to get or reuse clickhouse container");

        // DB connection string
        let dsn = format!("{}:{}", container_config.url(), container_config.port(),);

        // Get clickhouse client.
        let client = db_utils::get_clickhouse_client(dsn).await;

        // Init, reset, or reuse clickhouse db
        clickhouse::configure_reset_or_reuse_clickhouse_db(
            // &client,
            &container_config,
        )
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
}

impl TestEnv {
    pub fn teardown_ci(&self) -> Result<(), TestEnvError> {
        // Get docker util
        let mut docker_util = get_docker_util().expect("[TestEnv:CI]: Failed to get docker util");

        // Get Clickhouse container id
        let container_id = self.clickhouse_container_name();

        // Teardown clickhouse container
        docker_util
            .stop_container(container_id)
            .expect("[TestEnv:CI]: Failed to teardown clickhouse container");

        Ok(())
    }
}

pub(crate) fn get_docker_util() -> Result<DockerUtil, DockerError> {
    return match DockerUtil::new() {
        Ok(docker_util) => Ok(docker_util),
        Err(e) => Err(e),
    };
}
