use db_utils::prelude::ClickHouseClient;
use docker_utils::prelude::{ContainerConfig, DockerError, DockerUtil};

mod ci;
mod config;
mod env_ci;
mod env_prod;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EnvUtil {
    clickhouse_container_name: String,
    clickhouse_container_port: u16,
    dbg: bool,
}

impl EnvUtil {
    pub fn new() -> Self {
        Self {
            clickhouse_container_name: String::new(),
            clickhouse_container_port: 0,
            dbg: false,
        }
    }
    pub fn with_debug() -> Self {
        Self {
            clickhouse_container_name: String::new(),
            clickhouse_container_port: 0,
            dbg: true,
        }
    }
}

impl EnvUtil {
    pub fn set_clickhouse_container_name(&mut self, clickhouse_container_name: String) {
        self.clickhouse_container_name = clickhouse_container_name;
    }
    pub fn set_clickhouse_container_port(&mut self, clickhouse_container_port: u16) {
        self.clickhouse_container_port = clickhouse_container_port;
    }
    pub fn clickhouse_container_name(&self) -> &str {
        &self.clickhouse_container_name
    }
    pub fn clickhouse_container_port(&self) -> u16 {
        self.clickhouse_container_port
    }
}

impl EnvUtil {
    pub fn get_docker_util(&self) -> Result<DockerUtil, DockerError> {
        if self.dbg {
            return match DockerUtil::with_debug() {
                Ok(docker_util) => Ok(docker_util),
                Err(e) => Err(e),
            };
        }

        return match DockerUtil::new() {
            Ok(docker_util) => Ok(docker_util),
            Err(e) => Err(e),
        };
    }
    async fn get_clickhouse_client(
        &self,
        container_config: &ContainerConfig<'_>,
    ) -> ClickHouseClient {
        // DB connection string
        let dsn = format!(
            "{}:{}",
            container_config.url(),
            container_config.connection_port(),
        );

        // Get clickhouse client.
        db_utils::get_clickhouse_client(dsn).await
    }
}

impl EnvUtil {
    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[EnvUtil]: {}", s);
        }
    }
}
