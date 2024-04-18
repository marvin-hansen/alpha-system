mod clickhouse_container_config;

use crate::prelude::{DockerError, DockerUtil, TestEnvError};
use crate::test_env::clickhouse_container_config::clickhouse_container_config;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TestEnv {
    docker_util: DockerUtil,
    reuse_clickhouse: bool,
}

impl TestEnv {
    /// Create a new instance of `TestEnv` for Continuous Integration (CI).
    pub fn ci(reuse_clickhouse: bool) -> Self {
        // Get docker util
        let mut docker_util = get_docker_util().expect("[TestEnv:CI]: Failed to get docker util");

        // Get or reuse clickhouse container
        get_or_reuse_clickhouse_container(&mut docker_util, reuse_clickhouse)
            .expect("[TestEnv:CI]: Failed to get or reuse clickhouse container");

        // Init or reuse clickhouse db
        init_or_reuse_clickhouse_db(&docker_util)
            .expect("[TestEnv:CI]: Failed to init or reuse clickhouse db");

        Self {
            docker_util,
            reuse_clickhouse,
        }
    }
}

fn get_docker_util() -> Result<DockerUtil, DockerError> {
    return match DockerUtil::new() {
        Ok(docker_util) => Ok(docker_util),
        Err(e) => Err(e),
    };
}

fn get_or_reuse_clickhouse_container(
    docker_util: &mut DockerUtil,
    reuse_container: bool,
) -> Result<(), TestEnvError> {
    let container_config = clickhouse_container_config(reuse_container);

    return match docker_util.get_or_start_container_config(&container_config) {
        Ok(_) => Ok(()),
        Err(e) => Err(TestEnvError(e.to_string())),
    };
}

fn init_or_reuse_clickhouse_db(_docker_util: &DockerUtil) -> Result<(), TestEnvError> {
    Ok(())
}
