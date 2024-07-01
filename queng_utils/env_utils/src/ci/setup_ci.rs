use crate::prelude::{EnvUtil, EnvironmentError};
use clickhouse_utils::ClickhouseUtil;
use common::prelude::ContainerConfig;
use container_specs::api_proxy_container_config::api_proxy_container_config;
use container_specs::clickhouse_container_config::clickhouse_container_config;
use kaiko_utils::KaikoUtil;
use std::time::Duration;
use tokio::time::sleep;

impl EnvUtil {
    /// Create a new Continuous Integration (CI) `Environment`
    pub async fn setup_ci(&mut self) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("Get docker util");
        let docker_util = self.docker_util();

        self.dbg_print("Get api proxy container config");
        let api_proxy_container_config = api_proxy_container_config();

        self.dbg_print("Reuse api proxy container");
        let (_, _) = docker_util
            .get_or_start_container_config(&api_proxy_container_config)
            .expect("[TestEnv:CI]: Failed to get or reuse clickhouse container");

        // Give the api proxy container some extra time to complete booting up.
        sleep(Duration::from_secs(1)).await;

        self.dbg_print("Get clickhouse container config");
        let clickhouse_container_config = clickhouse_container_config();

        self.dbg_print("Reuse clickhouse container");
        let (_, _) = docker_util
            .get_or_start_container_config(&clickhouse_container_config)
            .expect("[TestEnv:CI]: Failed to get or reuse clickhouse container");

        // Give the container some extra time.
        sleep(Duration::from_secs(1)).await;

        self.dbg_print("Get clickhouse utils");
        let ch_utils = self
            .clickhouse_util()
            .await
            .expect("Failed to get ClickHouse Util");

        self.dbg_print("Get Kaiko util");
        let kaiko_util = self.kaiko_util();

        self.dbg_print("Configure clickhouse DB");
        self.configure_clickhouse(&ch_utils, &clickhouse_container_config, kaiko_util)
            .await
            .expect("Failed to configure clickhouse DB");

        Ok(())
    }

    pub(crate) async fn configure_clickhouse(
        &self,
        ch_utils: &ClickhouseUtil,
        container_config: &ContainerConfig<'_>,
        kaiko_util: &KaikoUtil,
    ) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("Check if clickhouse is already configured");
        if self.is_clickhouse_configured(container_config) {
            // Check if NO reset is required.
            self.dbg_print("Check if NO reset is required");
            if !container_config.reset_configuration()
                && self.is_data_set_current(ch_utils, kaiko_util)
            {
                // If so, abort & return.
                // Nothing to do in this case.
                self.dbg_print("Nothing to configure or reset; return.");
                return Ok(());
            }
        }

        // Check if the container configuration should be reset
        // or if the dataset is outdated. If so, delete everything.
        self.dbg_print("Check if reset is required if data are outdated");
        if container_config.reset_configuration() || !self.is_data_set_current(ch_utils, kaiko_util)
        {
            self.dbg_print("Drop all databases");
            ch_utils
                .teardown_db()
                .await
                .expect("[configure_clickhouse]: Failed to drop all databases")
        }

        // We know that the DB is either not configured or has been deleted
        // so we can re-crete all databases, tables, and import all data;
        self.dbg_print("Create all databases & tables");
        self.setup_db_and_tables(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to create all databases and tables");

        self.import_data(ch_utils, kaiko_util)
            .await
            .expect("[configure_clickhouse]: Failed to import data int Clickhouse");

        Ok(())
    }

    fn is_clickhouse_configured(&self, _container_config: &ContainerConfig) -> bool {
        // read stats table from CH and if there is no error, return true.
        false
    }

    fn is_data_set_current(&self, _ch_utils: &ClickhouseUtil, _kaiko_util: &KaikoUtil) -> bool {
        //
        // read stats table from CH and extract hash
        // compare hash from DB to hash from API
        // return true if they match, otherwise false.
        false
    }

    async fn setup_db_and_tables(&self, ch_utils: &ClickhouseUtil) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("Create all databases");
        ch_utils
            .setup_db()
            .await
            .expect("[setup_db_and_tables]: Failed to create all databases");

        self.dbg_print("Create all metadata tables");
        ch_utils
            .create_metadata_tables()
            .await
            .expect("[setup_db_and_tables]: Failed to create metadata tables");

        Ok(())
    }

    pub(crate) async fn import_data(
        &self,
        ch_utils: &ClickhouseUtil,
        kaiko_util: &KaikoUtil,
    ) -> Result<(), EnvironmentError> {
        //
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

        self.dbg_print("Download metadata statistic");
        let stats = kaiko_util
            .get_stats()
            .await
            .expect("Failed to get metadata statistic");

        self.dbg_print("Import metadata statistic");
        ch_utils
            .import_stats_metadata(&stats)
            .await
            .expect("Failed to import metadata statistic");

        Ok(())
    }
}
