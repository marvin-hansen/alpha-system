use crate::prelude::DockerUtil;

mod ci;
mod config;
mod env_ci;
mod env_prod;
mod shared;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EnvUtil {
    clickhouse_container_name: String,
    clickhouse_container_port: u16,
}

impl EnvUtil {
    pub fn docker_util(&self) -> DockerUtil {
        shared::get_docker_util().unwrap()
    }
    pub fn clickhouse_container_name(&self) -> &str {
        &self.clickhouse_container_name
    }
    pub fn clickhouse_container_port(&self) -> u16 {
        self.clickhouse_container_port
    }
}
