mod clickhouse_container_config;

use crate::prelude::{DockerError, DockerUtil, TestEnvError};
use crate::test_env::clickhouse_container_config::clickhouse_container_config;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TestEnv {
    docker_util: DockerUtil,
    clickhouse_container_name: String,
    clickhouse_container_port: u16,
}

impl TestEnv {
    /// Create a new instance of `TestEnv` for Continuous Integration (CI).
    pub fn setup_ci() -> Result<Self, TestEnvError> {
        // Get docker util
        let mut docker_util = get_docker_util().expect("[TestEnv:CI]: Failed to get docker util");

        // Get or reuse clickhouse container
        let container_config = clickhouse_container_config();

        let (clickhouse_container_name, clickhouse_container_port) = docker_util
            .get_or_start_container_config(&container_config)
            .expect("[TestEnv:CI]: Failed to get or reuse clickhouse container");

        // Init or reuse clickhouse db
        init_or_reuse_clickhouse_db(&docker_util)
            .expect("[TestEnv:CI]: Failed to init or reuse clickhouse db");

        Ok(Self {
            docker_util,
            clickhouse_container_name,
            clickhouse_container_port,
        })
    }

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

impl TestEnv {
    pub fn docker_util(&self) -> DockerUtil {
        self.docker_util
    }
    pub fn clickhouse_container_name(&self) -> &str {
        &self.clickhouse_container_name
    }
    pub fn clickhouse_container_port(&self) -> u16 {
        self.clickhouse_container_port
    }
}

fn get_docker_util() -> Result<DockerUtil, DockerError> {
    return match DockerUtil::new() {
        Ok(docker_util) => Ok(docker_util),
        Err(e) => Err(e),
    };
}

fn init_or_reuse_clickhouse_db(_docker_util: &DockerUtil) -> Result<(), TestEnvError> {
    Ok(())
}
