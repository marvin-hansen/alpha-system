use crate::prelude::{EnvUtil, EnvironmentError};
use clickhouse_utils::ClickhouseUtil;
use common::prelude::ContainerConfig;
use container_specs::clickhouse_container_config::clickhouse_container_config;
use kaiko_utils::KaikoUtil;
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

        self.dbg_print("Get or reuse clickhouse container");
        let (clickhouse_container_name, clickhouse_container_port) = docker_util
            .get_or_start_container_config(&container_config)
            .expect("[TestEnv:CI]: Failed to get or reuse clickhouse container");

        // Give the container some extra time to complete booting up.
        // Otherwise, you may get a connection refused error when connecting the client.
        sleep(Duration::from_millis(500));

        // Once the container is up & running, configure the DB
        self.dbg_print("Get clickhouse client");
        let client = self.get_clickhouse_client(&container_config).await;

        self.dbg_print("Get clickhouse utils");
        let ch_utils = self.get_clickhouse_util(client).await;

        self.dbg_print("Get Kaiko util");
        let kaiko_util = self
            .get_kaiko_util()
            .await
            .expect("Failed to get KaikoUtil");

        self.dbg_print("Configure clickhouse DB");
        self.configure_clickhouse(&ch_utils, &container_config, &kaiko_util)
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
        kaiko_util: &KaikoUtil,
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

        self.dbg_print("Create all metadata tables");
        ch_utils
            .create_metadata_tables()
            .await
            .expect("Failed to create metadata tables");

        self.dbg_print("Download assets metadata");
        let assets = kaiko_util.get_assets().await.expect("Failed to get assets");

        self.dbg_print("Import assets metadata");
        ch_utils
            .import_asset_metadata(&assets)
            .await
            .expect("Failed to import assets metadata");

        self.dbg_print("Download exchange metadata");
        let exchanges = kaiko_util
            .get_exchanges()
            .await
            .expect("Failed to get exchanges");

        self.dbg_print("Import exchanges metadata");
        ch_utils
            .import_exchanges_metadata(&exchanges)
            .await
            .expect("Failed to import exchanges metadata");

        self.dbg_print("Download instrument metadata");
        let instruments = kaiko_util
            .get_instruments()
            .await
            .expect("Failed to get instruments");

        self.dbg_print("Import instrument metadata");
        ch_utils
            .import_instruments_metadata(&instruments)
            .await
            .expect("Failed to import instrument metadata");

        Ok(())
    }

    pub(crate) fn is_clickhouse_configured(&self, _container_config: &ContainerConfig) -> bool {
        false
    }
}
