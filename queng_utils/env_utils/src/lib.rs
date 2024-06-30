use crate::prelude::EnvironmentError;
use clickhouse_utils::error::ClickHouseUtilError;
use clickhouse_utils::ClickhouseUtil;
use container_specs::api_proxy_container_config::api_proxy_container_config;
use container_specs::clickhouse_container_config::clickhouse_container_config;
use docker_utils::DockerUtil;
use kaiko_utils::KaikoUtil;

mod ci;

pub mod errors;
mod init;
pub mod prelude;

pub struct EnvUtil {
    api_proxy_container_name: String,
    api_proxy_container_port: u16,
    clickhouse_container_name: String,
    clickhouse_container_port: u16,
    containers_crated: bool,
    // ci_env_configured: bool,
    docker_util: DockerUtil,
    kaiko_util: KaikoUtil,
    dbg: bool,
}

impl EnvUtil {
    pub async fn new() -> Result<Self, EnvironmentError> {
        Self::build(false).await
    }

    pub async fn with_debug() -> Result<Self, EnvironmentError> {
        Self::build(true).await
    }

    async fn build(dbg: bool) -> Result<Self, EnvironmentError> {
        // Init configs
        let clickhouse_container_config = clickhouse_container_config();
        let api_proxy_container_config = api_proxy_container_config();
        // Init utils
        let mut docker_util =
            Self::init_docker_util(dbg).expect("EnvUtil: Failed to create Docker util");

        let kaiko_util = Self::init_kaiko_util(dbg)
            .await
            .expect("EnvUtil: Failed to get Kaiko util");

        // Init containers to check which one is initialized
        let (api_proxy_container_name, api_proxy_container_port, api_proxy_exists) =
            Self::init_container(&api_proxy_container_config, &mut docker_util)
                .expect("EnvUtil: Failed to init / verify api proxy container");

        let (clickhouse_container_name, clickhouse_container_port, clickhouse_exists) =
            Self::init_container(&clickhouse_container_config, &mut docker_util)
                .expect("EnvUtil: Failed to init / verify api proxy container");

        // set the boolean flag for all containers
        let containers_crated = api_proxy_exists && clickhouse_exists;

        Ok(Self {
            api_proxy_container_name,
            api_proxy_container_port,
            clickhouse_container_name,
            clickhouse_container_port,
            containers_crated,
            docker_util,
            kaiko_util,
            dbg,
        })
    }
}

impl EnvUtil {
    pub fn set_api_proxy_container_port(&mut self, api_proxy_container_port: u16) {
        self.api_proxy_container_port = api_proxy_container_port;
    }
    pub fn set_api_proxy_container_name(&mut self, api_proxy_container_name: String) {
        self.api_proxy_container_name = api_proxy_container_name;
    }
    pub fn api_proxy_container_port(&self) -> u16 {
        self.api_proxy_container_port
    }
    pub fn api_proxy_container_name(&self) -> &str {
        &self.api_proxy_container_name
    }
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
    pub fn containers_crated(&self) -> bool {
        self.containers_crated
    }
    pub fn set_containers_crated(&mut self) {
        self.containers_crated = true;
    }
    pub fn docker_util(&self) -> DockerUtil {
        self.docker_util
    }
    pub fn kaiko_util(&self) -> &KaikoUtil {
        &self.kaiko_util
    }
}

impl EnvUtil {
    pub(crate) async fn clickhouse_util(&self) -> Result<ClickhouseUtil, ClickHouseUtilError> {
        let container_config = clickhouse_container_config();

        // DB connection string
        let dsn = format!(
            "{}:{}",
            container_config.url(),
            container_config.connection_port(),
        );

        return if self.dbg {
            ClickhouseUtil::with_debug(dsn).await
        } else {
            ClickhouseUtil::new(dsn).await
        };
    }
}

impl EnvUtil {
    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[EnvUtil]: {}", s);
        }
    }
}
