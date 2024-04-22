mod ci;
mod config;

use crate::prelude::DockerUtil;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TestEnv {
    docker_util: DockerUtil,
    clickhouse_container_name: String,
    clickhouse_container_port: u16,
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
