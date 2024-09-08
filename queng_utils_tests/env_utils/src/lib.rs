use std::cell::RefCell;

use common_env::prelude::EnvironmentType;
use config_manager::CfgManager;
use docker_utils::prelude::DockerUtil;
use kaiko_utils::KaikoUtil;
use specs_utils::prelude::{
    api_proxy_container_specs, clickhouse_container_specs, postgres_db_container_specs,
};

use crate::prelude::EnvironmentError;

mod env;
pub mod errors;
mod getters;
mod getters_utils;
mod init;
pub mod prelude;

pub struct EnvUtil {
    env: EnvironmentType,
    api_proxy_container_name: RefCell<String>,
    api_proxy_container_port: RefCell<u16>,
    clickhouse_container_name: RefCell<String>,
    clickhouse_container_port: RefCell<u16>,
    postgres_db_container_name: RefCell<String>,
    postgres_db_container_port: RefCell<u16>,
    //
    all_containers_crated: RefCell<bool>,
    postgres_configured: RefCell<bool>,
    clickhouse_configured: RefCell<bool>,
    ci_env_configured: RefCell<bool>,
    //
    docker_util: DockerUtil,
    kaiko_util: KaikoUtil,
    dbg: bool,
}

impl EnvUtil {
    /// Creates a new `EnvUtil` instance.
    ///
    /// This function creates a new `EnvUtil` instance asynchronously.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `EnvUtil` instance
    /// or an `EnvironmentError` if an error occurs.
    ///
    pub async fn new() -> Result<Self, EnvironmentError> {
        Self::build(false).await
    }

    /// Creates a new `EnvUtil` instance with debug mode enabled.
    ///
    /// This function creates a new `EnvUtil` instance asynchronously with debug mode enabled.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `EnvUtil` instance
    /// or an `EnvironmentError` if an error occurs.
    pub async fn with_debug() -> Result<Self, EnvironmentError> {
        Self::build(true).await
    }

    /// Asynchronously builds and initializes an `EnvUtil` instance.
    ///
    /// This function is responsible for setting up the environment by detecting the current
    /// environment type, retrieving container configurations, and initializing various utilities
    /// and containers.
    ///
    /// # Arguments
    ///
    /// * `dbg` - A boolean flag indicating whether debug mode is enabled.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `EnvUtil` instance if successful,
    /// or an `EnvironmentError` if an error occurs.
    ///
    /// # Errors
    ///
    /// This function will return an `EnvironmentError` if it fails to initialize the Docker utility,
    /// Kaiko utility, or any of the specified containers.
    ///
    async fn build(dbg: bool) -> Result<Self, EnvironmentError> {
        // Autodetect the environment in which the system runs
        let env = CfgManager::detect_env_type(dbg);

        // Get container configs
        let clickhouse_container_config = clickhouse_container_specs();
        let api_proxy_container_config = api_proxy_container_specs();
        let postgres_db_container_config = postgres_db_container_specs();

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

        let (postgres_db_container_name, postgres_db_container_port, postgres_db_exists) =
            Self::init_container(&postgres_db_container_config, &docker_util)
                .expect("EnvUtil: Failed to init / verify api proxy container");

        // set the boolean flag for all containers
        let containers_crated = api_proxy_exists && clickhouse_exists && postgres_db_exists;

        let all_containers_crated = RefCell::new(containers_crated);
        let postgres_configured = RefCell::new(false);
        let clickhouse_configured = RefCell::new(false);
        let ci_env_configured = RefCell::new(false);

        let instance = Self {
            env,
            api_proxy_container_name: RefCell::new(api_proxy_container_name),
            api_proxy_container_port: RefCell::new(api_proxy_container_port),
            clickhouse_container_name: RefCell::new(clickhouse_container_name),
            clickhouse_container_port: RefCell::new(clickhouse_container_port),
            postgres_db_container_name: RefCell::new(postgres_db_container_name),
            postgres_db_container_port: RefCell::new(postgres_db_container_port),
            all_containers_crated,
            postgres_configured,
            clickhouse_configured,
            ci_env_configured,
            docker_util,
            kaiko_util,
            dbg,
        };

        if containers_crated {
            let clickhouse_configured = instance.verify_clickhouse_db().await.is_ok();

            let postgres_configured = instance.verify_postgres_db().await.is_ok();

            if clickhouse_configured {
                instance.clickhouse_configured.replace(true);
            }

            if postgres_configured {
                instance.postgres_configured.replace(true);
            }

            if clickhouse_configured && postgres_configured {
                instance.ci_env_configured.replace(true);
            }
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
