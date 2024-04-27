use crate::env_util::config::clickhouse_container_config::clickhouse_container_config;
use crate::env_util::config::meta_data_import_config::meta_data_import_config;

use crate::env_util::EnvUtil;
use crate::prelude::EnvironmentError;
use clickhouse_utils::prelude::DataImportConfig;
use clickhouse_utils::ClickhouseUtil;
use docker_utils::prelude::ContainerConfig;
use std::thread::sleep;
use std::time::Duration;

impl EnvUtil {
    /// Create a new Continuous Integration (CI) `Environment`
    pub async fn setup_ci(&mut self) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("Get docker util");
        let mut docker_util = self
            .get_docker_util()
            .expect("[TestEnv:CI]: Failed to get docker util");

        self.dbg_print("Get clickhouse container config");
        let container_config = clickhouse_container_config();

        self.dbg_print("Get meta data import config");
        let meta_data_import_config = meta_data_import_config();

        self.dbg_print("Get or reuse clickhouse container");
        let (clickhouse_container_name, clickhouse_container_port) = docker_util
            .get_or_start_container_config(&container_config)
            .expect("[TestEnv:CI]: Failed to get or reuse clickhouse container");

        // Give the container some extra time to complete booting up.
        // Otherwise, you may get a connection refused error when connecting the client.
        sleep(Duration::from_millis(100));

        // Once the container is up & running, configure the DB
        self.dbg_print("Get clickhouse client");
        let client = self.get_clickhouse_client(&container_config).await;

        self.dbg_print("Get clickhouse utils");
        let ch_utils = self.get_clickhouse_util(client).await;

        self.dbg_print("Configure clickhouse DB");
        self.configure_clickhouse(&ch_utils, &container_config, &meta_data_import_config)
            .await
            .expect("Failed to configure clickhouse DB");

        self.dbg_print("Set container name and port");
        self.set_clickhouse_container_name(clickhouse_container_name);
        self.set_clickhouse_container_port(clickhouse_container_port);
        Ok(())
    }

    pub(crate) async fn configure_clickhouse(
        &self,
        ch_utils: &ClickhouseUtil,
        container_config: &ContainerConfig<'_>,
        meta_data_import_config: &DataImportConfig<'_>,
    ) -> Result<(), EnvironmentError> {
        // Check if DB is already configured
        let configured = self.is_clickhouse_configured(container_config);
        if configured {
            return Ok(());
        }

        // Check if the container configuration should be re-set
        let reset_config = container_config.reset_configuration();
        if reset_config {
            self.dbg_print("Drop all databases");
            ch_utils
                .teardown_db()
                .await
                .expect("[configure_clickhouse]: Failed to drop all databases")
        }

        self.dbg_print("Create all databases");
        ch_utils
            .setup_db()
            .await
            .expect("[configure_clickhouse]: Failed to create all databases");

        self.dbg_print("Create all mete data tables");
        ch_utils
            .create_metadata_tables()
            .await
            .expect("Failed to create meta data tables");

        // Import meta data
        ch_utils
            .import_all_data(
                meta_data_import_config.assets_data_path(),
                meta_data_import_config.exchanges_data_path(),
                meta_data_import_config.instruments_data_path(),
            )
            .await
            .expect("");

        Ok(())
    }

    pub(crate) fn is_clickhouse_configured(&self, _container_config: &ContainerConfig) -> bool {
        false
    }
}
