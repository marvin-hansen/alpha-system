use crate::prelude::{EnvUtil, EnvironmentError};
use clickhouse_utils::ClickhouseUtil;
use common::prelude::ContainerConfig;
use container_specs::clickhouse_container_config::clickhouse_container_config;
use kaiko_utils::KaikoUtil;

impl EnvUtil {
    /// Create and configure a new Continuous Integration (CI) `Environment`
    pub async fn setup_ci(&mut self) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[setup_ci]: Set data sample size to 10%");
        let sample_size = None; // Some(0);
                                //
        self.dbg_print("[setup_ci]: Get or reuse all containers");
        self.setup_containers()
            .await
            .expect("[setup_ci]: Failed to setup containers");

        self.dbg_print("[setup_ci]: Get clickhouse container config");
        let clickhouse_container_config = clickhouse_container_config();

        self.dbg_print("[setup_ci]: Get clickhouse utils");
        let ch_utils = self
            .clickhouse_util()
            .await
            .expect("[setup_ci]: Failed to get ClickHouse Util");

        self.dbg_print("Get Kaiko util");
        let kaiko_util = self.kaiko_util();

        self.dbg_print("[setup_ci]: Configure clickhouse DB");
        self.configure_clickhouse(
            &ch_utils,
            &clickhouse_container_config,
            kaiko_util,
            sample_size,
        )
        .await
        .expect("[setup_ci]: Failed to configure clickhouse DB");

        self.verify_clickhouse(&ch_utils, kaiko_util, sample_size)
            .await
            .expect("[setup_ci]: Failed to verify clickhouse DB");

        Ok(())
    }

    async fn configure_clickhouse(
        &self,
        ch_utils: &ClickhouseUtil,
        container_config: &ContainerConfig<'_>,
        kaiko_util: &KaikoUtil,
        sample_size: Option<u32>,
    ) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[configure_clickhouse]: Check if clickhouse is already configured");
        let tables_created = self
            .verify_tables_created(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to check if all database tables configured");

        self.dbg_print("[configure_clickhouse]: Check if all clickhouse data are already imported");
        let imported = self
            .verify_import_data(ch_utils, kaiko_util, None)
            .await
            .expect("[configure_clickhouse]: Failed to check if all data imported");

        if tables_created && imported {
            // Check if NO reset is required.
            self.dbg_print("[configure_clickhouse]: Check if NO reset is required");
            if !container_config.reset_configuration() {
                // If so, abort & return. Nothing to do in this case.
                self.dbg_print(
                    "[configure_clickhouse]: Nothing to configure, import, or reset; return.",
                );
                return Ok(());
            }
        }

        // Check if the container configuration should be reset. If so, delete everything.
        self.dbg_print("[configure_clickhouse]: Check if reset is required if data are outdated");
        if container_config.reset_configuration() {
            self.dbg_print("[configure_clickhouse]: Drop all databases");
            ch_utils
                .teardown_db()
                .await
                .expect("[configure_clickhouse]: Failed to drop all databases")
        }

        // We know that the DB is either not configured or has been deleted
        // so we can re-crete all databases, tables, and import all data;
        self.dbg_print("[configure_clickhouse]: Create all databases");
        self.setup_db(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to create all databases");

        self.dbg_print("[configure_clickhouse]: Create all tables");
        self.create_tables(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to create all tables");

        self.dbg_print("[configure_clickhouse]: Verify that all tables are created");
        let tables_created = self
            .verify_tables_created(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to verify if all tables are created");
        assert!(tables_created);

        self.dbg_print("[configure_clickhouse]: Import data into clickhouse");
        self.import_metadata(ch_utils, kaiko_util, sample_size)
            .await
            .expect("[configure_clickhouse]: Failed to import data into Clickhouse");
        Ok(())
    }

    async fn setup_db(&self, ch_utils: &ClickhouseUtil) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[setup_db]: Create all databases");
        ch_utils
            .setup_db()
            .await
            .expect("[setup_db]: Failed to create all databases");

        Ok(())
    }

    async fn create_tables(&self, ch_utils: &ClickhouseUtil) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[create_tables]:Create all metadata tables");
        ch_utils
            .create_metadata_tables()
            .await
            .expect("[create_tables]: Failed to create metadata tables");

        Ok(())
    }

    async fn import_metadata(
        &self,
        ch_utils: &ClickhouseUtil,
        kaiko_util: &KaikoUtil,
        _sample_size: Option<u32>,
    ) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[import_data]: Download assets metadata");
        let assets = kaiko_util
            .get_assets()
            .await
            .expect("[import_data]: Failed to get assets");

        self.dbg_print("[import_data]: Import assets metadata");
        ch_utils
            .metadata
            .import_asset_metadata(&assets)
            .await
            .expect("[import_data]: Failed to import assets metadata");

        self.dbg_print("[import_data]: Download exchange metadata");
        let exchanges = kaiko_util
            .get_exchanges()
            .await
            .expect("[import_data]: Failed to get exchanges");

        self.dbg_print("[import_data]: Import exchanges metadata");
        ch_utils
            .metadata
            .import_exchanges_metadata(&exchanges)
            .await
            .expect("[import_data]: Failed to import exchanges metadata");

        self.dbg_print("[import_data]: Download instrument metadata");
        let instruments = kaiko_util
            .get_instruments()
            .await
            .expect("[import_data]: Failed to get instruments");

        self.dbg_print("[import_data]: Import instrument metadata");
        ch_utils
            .metadata
            .import_instruments_metadata(&instruments)
            .await
            .expect("[import_data]: Failed to import instrument metadata");

        self.dbg_print("[import_data]: Download metadata statistic");
        let stats = kaiko_util
            .get_stats()
            .await
            .expect("[import_data]: Failed to get metadata statistic");

        self.dbg_print("[import_data]: Import metadata statistic");
        ch_utils
            .metadata
            .import_stats_metadata(&stats)
            .await
            .expect("[import_data]: Failed to import metadata statistic");

        Ok(())
    }

    async fn verify_clickhouse(
        &self,
        ch_utils: &ClickhouseUtil,
        kaiko_util: &KaikoUtil,
        _sample_size: Option<u32>,
    ) -> Result<(), EnvironmentError> {
        self.dbg_print("[verify_clickhouse]: Check if clickhouse is already configured");
        let tables_created = self
            .verify_tables_created(ch_utils)
            .await
            .expect("[verify_clickhouse]: Failed to check if all database tables configured");

        if !tables_created {
            return Err(EnvironmentError::from(
                "[verify_clickhouse]: Error: Tables were not created.",
            ));
        }

        self.dbg_print("[verify_clickhouse]: Verify that all data were imported");
        self.verify_import_data(ch_utils, kaiko_util, None)
            .await
            .expect("[verify_clickhouse]: Failed to verify data import Clickhouse");

        Ok(())
    }

    async fn verify_tables_created(
        &self,
        ch_utils: &ClickhouseUtil,
    ) -> Result<bool, EnvironmentError> {
        //
        self.dbg_print("[verify_tables_created]: Check if all metadata tables exist");
        let exists_metadata_tables = match ch_utils.verify_all_metadata_tables().await {
            Ok(exists) => exists,
            Err(e) => return Err(EnvironmentError::from(e.to_string())),
        };

        let all_exists = exists_metadata_tables;

        return Ok(all_exists);
    }

    async fn verify_import_data(
        &self,
        ch_utils: &ClickhouseUtil,
        kaiko_util: &KaikoUtil,
        _sample_size: Option<u32>,
    ) -> Result<bool, EnvironmentError> {
        //
        self.dbg_print("[verify_import_data]: Check if all data imported into the metadata DB");
        let stats = kaiko_util
            .get_stats()
            .await
            .expect("[verify_import_data]: Failed to get metadata statistic");

        self.dbg_print("[verify_import_data]: Fetch reference metadata from API Proxy");
        let nr_stats_assets = stats.number_assets() as u64;
        let nr_stats_exchanges = stats.number_exchanges() as u64;
        let nr_stats_instruments = stats.number_instruments() as u64;

        self.dbg_print("[verify_import_data]: Count assets metadata in DB");
        let nr_db_assets = ch_utils
            .metadata
            .count_assets()
            .await
            .expect("[verify_import_data]: Failed to get count assets from metadata DB");

        self.dbg_print("[verify_import_data]: Count exchanges metadata in DB");
        let nr_db_exchanges = ch_utils
            .metadata
            .count_exchanges()
            .await
            .expect("[verify_import_data]: Failed to get count exchanges from metadata DB");

        self.dbg_print("[verify_import_data]: Count instruments metadata in DB");
        let nr_db_instruments = ch_utils
            .metadata
            .count_instruments()
            .await
            .expect("[verify_import_data]: Failed to get count instruments from metadata DB");

        let assets_imported = nr_stats_assets == nr_db_assets;
        self.dbg_print(&format!(
            "[verify_import_data]: All assets imported: {}",
            assets_imported
        ));

        let exchanges_imported = nr_stats_exchanges == nr_db_exchanges;
        self.dbg_print(&format!(
            "[verify_import_data]: All exchanges imported: {}",
            exchanges_imported
        ));

        let instruments_imported = nr_stats_instruments == nr_db_instruments;
        self.dbg_print(&format!(
            "[verify_import_data]: All instruments imported: {}",
            instruments_imported
        ));

        let all_imported = assets_imported && exchanges_imported && instruments_imported;
        self.dbg_print(&format!(
            "[verify_import_data]: All data imported: {}",
            all_imported
        ));

        Ok(all_imported)
    }
}
