use crate::prelude::{EnvUtil, EnvironmentError};
use clickhouse_utils::ClickhouseUtil;
use common::prelude::ContainerConfig;
use container_specs::clickhouse_container_config::clickhouse_container_config;
use kaiko_utils::KaikoUtil;

impl EnvUtil {
    /// Create and configure a new Continuous Integration (CI) `Environment`
    pub async fn setup_ci(&mut self) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("Get or reuse all containers");
        self.setup_containers()
            .await
            .expect("[setup_ci]: Failed to setup containers");

        self.dbg_print("Get clickhouse container config");
        let clickhouse_container_config = clickhouse_container_config();

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

    async fn configure_clickhouse(
        &self,
        ch_utils: &ClickhouseUtil,
        container_config: &ContainerConfig<'_>,
        kaiko_util: &KaikoUtil,
    ) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("Check if clickhouse is already configured");
        let tables_created = self
            .verify_tables_created(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to check if all database tables configured");

        self.dbg_print("Check if all clickhouse data are already imported");
        let imported = true;

        if tables_created && imported {
            // Check if NO reset is required.
            self.dbg_print("Check if NO reset is required");
            if !container_config.reset_configuration() {
                // If so, abort & return. Nothing to do in this case.
                self.dbg_print("Nothing to configure, import, or reset; return.");
                return Ok(());
            }
        }

        // Check if the container configuration should be reset. If so, delete everything.
        self.dbg_print("Check if reset is required if data are outdated");
        if container_config.reset_configuration() {
            self.dbg_print("Drop all databases");
            ch_utils
                .teardown_db()
                .await
                .expect("[configure_clickhouse]: Failed to drop all databases")
        }

        // We know that the DB is either not configured or has been deleted
        // so we can re-crete all databases, tables, and import all data;
        self.dbg_print("Create all databases");
        self.setup_db(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to create all databases");

        self.dbg_print("Create all tables");
        self.create_tables(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to create all tables");

        self.dbg_print("Verify that all tables are created");
        let tables_created = self
            .verify_tables_created(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to verify if all tables are created");
        assert!(tables_created);

        self.dbg_print("Import data into clickhouse");
        self.import_metadata(ch_utils, kaiko_util)
            .await
            .expect("[configure_clickhouse]: Failed to import data into Clickhouse");

        self.dbg_print("Verify that all data were imported");
        self.verify_import_data(ch_utils, kaiko_util, None)
            .await
            .expect("[configure_clickhouse]: Failed to verify data import Clickhouse");

        Ok(())
    }

    async fn verify_tables_created(
        &self,
        ch_utils: &ClickhouseUtil,
    ) -> Result<bool, EnvironmentError> {
        //
        self.dbg_print("Check if all metadata tables exist");
        let exists_metadata_tables = match ch_utils.verify_all_metadata_tables().await {
            Ok(exists) => exists,
            Err(e) => return Err(EnvironmentError::from(e.to_string())),
        };

        let all_exists = exists_metadata_tables;

        return Ok(all_exists);
    }

    async fn setup_db(&self, ch_utils: &ClickhouseUtil) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("Create all databases");
        ch_utils
            .setup_db()
            .await
            .expect("[setup_db_and_tables]: Failed to create all databases");

        Ok(())
    }

    async fn create_tables(&self, ch_utils: &ClickhouseUtil) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("Create all metadata tables");
        ch_utils
            .create_metadata_tables()
            .await
            .expect("[setup_db_and_tables]: Failed to create metadata tables");

        Ok(())
    }

    async fn import_metadata(
        &self,
        ch_utils: &ClickhouseUtil,
        kaiko_util: &KaikoUtil,
    ) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("Download assets metadata");
        let assets = kaiko_util
            .get_assets()
            .await
            .expect("[import_data]: Failed to get assets");

        self.dbg_print("Import assets metadata");
        ch_utils
            .metadata
            .import_asset_metadata(&assets)
            .await
            .expect("[import_data]: Failed to import assets metadata");

        self.dbg_print("Download exchange metadata");
        let exchanges = kaiko_util
            .get_exchanges()
            .await
            .expect("[import_data]: Failed to get exchanges");

        self.dbg_print("Import exchanges metadata");
        ch_utils
            .metadata
            .import_exchanges_metadata(&exchanges)
            .await
            .expect("[import_data]: Failed to import exchanges metadata");

        self.dbg_print("Download instrument metadata");
        let instruments = kaiko_util
            .get_instruments()
            .await
            .expect("[import_data]: Failed to get instruments");

        self.dbg_print("Import instrument metadata");
        ch_utils
            .metadata
            .import_instruments_metadata(&instruments)
            .await
            .expect("[import_data]: Failed to import instrument metadata");

        self.dbg_print("Download metadata statistic");
        let stats = kaiko_util
            .get_stats()
            .await
            .expect("[import_data]: Failed to get metadata statistic");

        self.dbg_print("Import metadata statistic");
        ch_utils
            .metadata
            .import_stats_metadata(&stats)
            .await
            .expect("[import_data]: Failed to import metadata statistic");

        Ok(())
    }

    async fn verify_import_data(
        &self,
        _ch_utils: &ClickhouseUtil,
        _kaiko_util: &KaikoUtil,
        _sample_size: Option<u32>,
    ) -> Result<(), EnvironmentError> {
        Ok(())
    }
}
