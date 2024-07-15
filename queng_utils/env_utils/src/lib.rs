use crate::prelude::EnvironmentError;

use docker_utils::DockerUtil;
use kaiko_utils::KaikoUtil;
use specs_utils::prelude::{
    api_proxy_container_specs, clickhouse_container_specs, surreal_db_container_specs,
};

mod env;
pub mod errors;
mod getters;
mod init;
pub mod prelude;

pub struct EnvUtil {
    //
    // env: Env,
    api_proxy_container_name: String,
    api_proxy_container_port: u16,
    clickhouse_container_name: String,
    clickhouse_container_port: u16,
    surreal_db_container_name: String,
    surreal_db_container_port: u16,
    //
    all_containers_crated: bool,
    ci_env_configured: bool,
    //
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
        // Get container configs
        let clickhouse_container_config = clickhouse_container_specs();
        let api_proxy_container_config = api_proxy_container_specs();
        let surreal_db_container_config = surreal_db_container_specs();

        // Build utils
        let docker_util =
            Self::init_docker_util(dbg).expect("EnvUtil: Failed to create Docker util");

        let kaiko_util = Self::init_kaiko_util(dbg)
            .await
            .expect("EnvUtil: Failed to get Kaiko util");

        // Init containers to check which one is initialized
        let (api_proxy_container_name, api_proxy_container_port, api_proxy_exists) =
            Self::init_container(&api_proxy_container_config, &docker_util)
                .expect("EnvUtil: Failed to init / verify api proxy container");

        let (clickhouse_container_name, clickhouse_container_port, clickhouse_exists) =
            Self::init_container(&clickhouse_container_config, &docker_util)
                .expect("EnvUtil: Failed to init / verify api proxy container");

        let (surreal_db_container_name, surreal_db_container_port, surreal_db_exists) =
            Self::init_container(&surreal_db_container_config, &docker_util)
                .expect("EnvUtil: Failed to init / verify api proxy container");

        // set the boolean flag for all containers
        let containers_crated = api_proxy_exists && clickhouse_exists && surreal_db_exists;
        let ci_env_configured = false;

        let mut instance = Self {
            api_proxy_container_name,
            api_proxy_container_port,
            clickhouse_container_name,
            clickhouse_container_port,
            surreal_db_container_name,
            surreal_db_container_port,
            all_containers_crated: containers_crated,
            ci_env_configured,
            docker_util,
            kaiko_util,
            dbg,
        };

        if containers_crated {
            match instance.verify_clickhouse().await {
                Ok(ci_env_configured) => {
                    if ci_env_configured {
                        instance.set_ci_env_configured(ci_env_configured);
                    }
                }
                Err(_) => {}
            };
        }

        Ok(instance)
    }
}

impl EnvUtil {
    fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[EnvUtil]: {}", s);
        }
    }
}
