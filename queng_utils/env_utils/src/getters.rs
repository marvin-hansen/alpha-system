use crate::EnvUtil;
use clickhouse_utils::prelude::ClickHouseUtilError;
use clickhouse_utils::ClickhouseUtil;
use container_specs::clickhouse_container_specs::clickhouse_container_config;
use docker_utils::DockerUtil;
use kaiko_utils::KaikoUtil;

impl EnvUtil {
    pub(super) async fn get_new_clickhouse_util(
        &self,
    ) -> Result<ClickhouseUtil, ClickHouseUtilError> {
        let container_config = clickhouse_container_config();

        // DB connection string
        let dsn = format!(
            "{}:{}",
            container_config.url(),
            container_config.connection_port(),
        );

        if self.dbg {
            ClickhouseUtil::with_debug(dsn).await
        } else {
            ClickhouseUtil::new(dsn).await
        }
    }
}

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
    pub fn containers_crated(&self) -> bool {
        self.containers_crated
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
    pub fn set_containers_crated(&mut self) {
        self.containers_crated = true;
    }
    pub fn set_ci_env_configured(&mut self, ci_env_configured: bool) {
        self.ci_env_configured = ci_env_configured;
    }
}
