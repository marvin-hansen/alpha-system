use common_env::prelude::EnvironmentType;
use docker_utils::DockerUtil;
use kaiko_utils::KaikoUtil;

use crate::EnvUtil;

impl EnvUtil {
    pub fn api_proxy_container_port(&self) -> u16 {
        self.api_proxy_container_port
    }
    pub fn api_proxy_container_name(&self) -> &str {
        &self.api_proxy_container_name
    }
    pub fn clickhouse_container_name(&self) -> &str {
        &self.clickhouse_container_name
    }
    pub fn clickhouse_container_port(&self) -> u16 {
        self.clickhouse_container_port
    }
    pub fn postgres_db_container_port(&self) -> u16 {
        self.postgres_db_container_port
    }
    pub fn postgres_db_container_name(&self) -> &str {
        &self.postgres_db_container_name
    }
    pub fn all_containers_crated(&self) -> bool {
        self.all_containers_crated
    }
    pub fn docker_util(&self) -> DockerUtil {
        self.docker_util
    }
    pub fn kaiko_util(&self) -> &KaikoUtil {
        &self.kaiko_util
    }
    pub fn ci_env_configured(&self) -> bool {
        self.ci_env_configured
    }
    pub fn env(&self) -> EnvironmentType {
        self.env
    }
    pub fn dbg(&self) -> bool {
        self.dbg
    }
}

impl EnvUtil {
    pub fn set_api_proxy_container_port(&mut self, api_proxy_container_port: u16) {
        self.api_proxy_container_port = api_proxy_container_port;
    }
    pub fn set_api_proxy_container_name(&mut self, api_proxy_container_name: String) {
        self.api_proxy_container_name = api_proxy_container_name;
    }
    pub fn set_clickhouse_container_name(&mut self, clickhouse_container_name: String) {
        self.clickhouse_container_name = clickhouse_container_name;
    }
    pub fn set_clickhouse_container_port(&mut self, clickhouse_container_port: u16) {
        self.clickhouse_container_port = clickhouse_container_port;
    }
    pub fn set_postgres_db_container_port(&mut self, postgres_db_container_port: u16) {
        self.postgres_db_container_port = postgres_db_container_port;
    }
    pub fn set_postgres_db_container_name(&mut self, postgres_db_container_name: String) {
        self.postgres_db_container_name = postgres_db_container_name;
    }
    pub fn set_all_containers_crated(&mut self) {
        self.all_containers_crated = true;
    }
    pub fn set_ci_env_configured(&mut self, ci_env_configured: bool) {
        self.ci_env_configured = ci_env_configured;
    }
}
