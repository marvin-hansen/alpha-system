use clickhouse_utils::ClickhouseUtil;
use kaiko_utils::KaikoUtil;

use crate::prelude::EnvironmentError;
use crate::EnvUtil;

impl EnvUtil {
    /// Verifies if all databases associated with the ClickHouseUtil object are created.
    ///
    /// This function verifies the specs database by calling the `verify_all_db_exists` method on the `specs` object.
    /// It returns `Ok(true)` if all databases are successfully verified, and `Err(EnvironmentError)` if any verification fails.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that indicates whether all databases have been verified.
    /// If successful, it returns `Ok(true)`.
    /// If an error occurs, it returns `Err(EnvironmentError)`.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `EnvironmentError`.
    ///
    pub(crate) async fn verify_clickhouse_db(&self) -> Result<bool, EnvironmentError> {
        //
        self.dbg_print("Get Clickhouse util");
        let ch_utils = &self
            .get_new_clickhouse_util()
            .await
            .expect("Failed to get ClickhouseUtil");

        self.dbg_print("Get Kaiko util");
        let kaiko_util = self.kaiko_util();

        self.dbg_print("[verify_clickhouse]: Check if all databases are created");
        let dbs_created = ch_utils
            .verify_all_db_exists()
            .await
            .expect("[verify_clickhouse]: Failed to check if all databases were created");

        if !dbs_created {
            return Err(EnvironmentError::from(
                "[verify_clickhouse]: Error: Databases were not created.",
            ));
        }

        self.dbg_print("[verify_clickhouse]: Verify that all data were imported");
        let data_imported = self
            .verify_clickhouse_data_imported(ch_utils, kaiko_util, None)
            .await
            .expect("[verify_clickhouse]: Failed to verify data import Clickhouse");

        if !data_imported {
            return Err(EnvironmentError::from(
                "[verify_clickhouse]: Error: Data were not imported.",
            ));
        }

        Ok(true)
    }

    /// Asynchronously verifies if all data has been imported into the ClickHouse database.
    ///
    /// This method performs the following steps:
    ///
    /// 1. Verifies if all metadata data exist in the ClickHouse metadata database.
    /// 2. Verifies if all specs data exist in the ClickHouse specs database.
    ///
    /// # Arguments
    ///
    /// * `ch_utils` - A reference to a `ClickhouseUtil` object.
    /// * `kaiko_util` - A reference to a `KaikoUtil` object.
    /// * `sample_size` - An optional `u32` value representing the sample size.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that indicates whether all data has been imported into the ClickHouse database.
    /// If successful, it returns `Ok(bool)` indicating whether all data has been imported.
    /// If an error occurs, it returns `Err(EnvironmentError)`.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `EnvironmentError`.
    ///
    pub(crate) async fn verify_clickhouse_data_imported(
        &self,
        ch_utils: &ClickhouseUtil,
        kaiko_util: &KaikoUtil,
        sample_size: Option<u32>,
    ) -> Result<bool, EnvironmentError> {
        let metadata_imported = self
            .verify_metadata_data_imported(ch_utils, kaiko_util, sample_size)
            .await
            .expect("[check_if_all_data_imported]: Failed to check if all metadata imported");

        let all_imported = metadata_imported;

        Ok(all_imported)
    }

    /// Verifies that all data have been imported into the metadata database.
    ///
    /// This method checks if all data have been imported into the metadata database.
    /// It performs the following steps:
    ///
    /// 1. Retrieves the `ClickhouseUtil` object.
    ///
    /// 2. Retrieves the `KaikoUtil` object.
    ///
    /// 3. Counts the number of assets, exchanges, and instruments in the metadata tables.
    ///
    /// 4. Compares the counts with predefined thresholds to determine if all data have been imported.
    ///
    /// # Arguments
    ///
    /// * `ch_utils` - A reference to a `ClickhouseUtil` object.
    ///
    /// * `kaiko_util` - A reference to a `KaikoUtil` object.
    ///
    /// * `sample_size` - An optional `u32` value representing the sample size.
    ///
    /// # Returns
    ///
    /// - `Ok(true)` if all data have been imported.
    /// - `Ok(false)` if any data table is empty.
    /// - `Err(EnvironmentError)` if an error occurs during the verification process.
    ///
    async fn verify_metadata_data_imported(
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

        self.dbg_print(&format!(
            "[verify_import_data]: Counted imported assets: {}",
            nr_db_assets
        ));

        self.dbg_print(&format!(
            "[verify_import_data]: API reference assets: {}",
            nr_stats_assets
        ));

        let assets_imported = nr_stats_assets == nr_db_assets;
        self.dbg_print(&format!(
            "[verify_import_data]: All assets imported: {}",
            assets_imported
        ));

        self.dbg_print("[verify_import_data]: Count exchanges metadata in DB");
        let nr_db_exchanges = ch_utils
            .metadata
            .count_exchanges()
            .await
            .expect("[verify_import_data]: Failed to get count exchanges from metadata DB");

        self.dbg_print(&format!(
            "[verify_import_data]: Counted imported exchanges: {}",
            nr_db_exchanges
        ));

        self.dbg_print(&format!(
            "[verify_import_data]: API reference exchanges: {}",
            nr_stats_exchanges
        ));

        let exchanges_imported = nr_stats_exchanges == nr_db_exchanges;
        self.dbg_print(&format!(
            "[verify_import_data]: All exchanges imported: {}",
            exchanges_imported
        ));

        self.dbg_print("[verify_import_data]: Count instruments metadata in DB");
        let nr_db_instruments = ch_utils
            .metadata
            .count_instruments()
            .await
            .expect("[verify_import_data]: Failed to get count instruments from metadata DB");

        self.dbg_print(&format!(
            "[verify_import_data]: Counted imported instruments: {}",
            nr_db_instruments
        ));

        self.dbg_print(&format!(
            "[verify_import_data]: API reference instruments: {}",
            nr_stats_instruments
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
