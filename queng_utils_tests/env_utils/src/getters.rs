use common_env::prelude::EnvironmentType;
use docker_utils::prelude::DockerUtil;
use kaiko_utils::KaikoUtil;

use crate::EnvUtil;

impl EnvUtil {
    pub fn api_proxy_container_port(&self) -> u16 {
        self.api_proxy_container_port.take()
    }
    pub fn api_proxy_container_name(&self) -> String {
        self.api_proxy_container_name.borrow().clone()
    }
    pub fn clickhouse_container_name(&self) -> String {
        self.clickhouse_container_name.borrow().clone()
    }
    pub fn clickhouse_container_port(&self) -> u16 {
        self.clickhouse_container_port.take()
    }
    pub fn postgres_db_container_port(&self) -> u16 {
        self.postgres_db_container_port.take()
    }
    pub fn postgres_db_container_name(&self) -> String {
        self.postgres_db_container_name.borrow().clone()
    }
    pub fn all_containers_crated(&self) -> bool {
        self.all_containers_crated.take()
    }
    pub fn docker_util(&self) -> DockerUtil {
        self.docker_util
    }
    pub fn kaiko_util(&self) -> &KaikoUtil {
        &self.kaiko_util
    }
    pub fn ci_env_configured(&self) -> bool {
        self.ci_env_configured.take()
    }
    pub fn env(&self) -> EnvironmentType {
        self.env
    }
    pub fn dbg(&self) -> bool {
        self.dbg
    }

    pub fn postgres_configured(&self) -> bool {
        self.postgres_configured.take()
    }

    pub fn clickhouse_configured(&self) -> bool {
        self.clickhouse_configured.take()
    }
}

impl EnvUtil {
    pub fn set_api_proxy_container_port(&self, api_proxy_container_port: u16) {
        self.api_proxy_container_port
            .replace(api_proxy_container_port);
    }
    pub fn set_api_proxy_container_name(&self, api_proxy_container_name: String) {
        self.api_proxy_container_name
            .replace(api_proxy_container_name);
    }
    pub fn set_clickhouse_container_name(&self, clickhouse_container_name: String) {
        self.clickhouse_container_name
            .replace(clickhouse_container_name);
    }
    pub fn set_clickhouse_container_port(&self, clickhouse_container_port: u16) {
        self.clickhouse_container_port
            .replace(clickhouse_container_port);
    }
    pub fn set_postgres_db_container_port(&self, postgres_db_container_port: u16) {
        self.postgres_db_container_port
            .replace(postgres_db_container_port);
    }
    pub fn set_postgres_db_container_name(&self, postgres_db_container_name: String) {
        self.postgres_db_container_name
            .replace(postgres_db_container_name);
    }
    pub fn set_all_containers_crated(&self) {
        self.all_containers_crated.replace(true);
    }
    pub fn set_ci_env_configured(&self, ci_env_configured: bool) {
        self.ci_env_configured.replace(ci_env_configured);
    }
}
