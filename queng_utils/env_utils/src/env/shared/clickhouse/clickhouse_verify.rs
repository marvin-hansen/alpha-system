use crate::prelude::EnvironmentError;
use crate::EnvUtil;
use clickhouse_utils::ClickhouseUtil;
use kaiko_utils::KaikoUtil;

impl EnvUtil {
    /// Verifies that the ClickHouse database is configured correctly.
    ///
    /// This function performs the following steps:
    ///
    /// 1. Checks if all database tables have been created.
    /// 2. Verifies that all data have been imported.
    ///
    /// # Arguments
    ///
    /// * `ch_utils` - A reference to a `ClickhouseUtil` object.
    /// * `kaiko_util` - A reference to a `KaikoUtil` object.
    /// * `sample_size` - An optional `u32` representing the sample size.
    ///
    /// # Errors
    ///
    /// - `EnvironmentError` if any step fails.
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
        let dbs_created = self
            .verify_clickhouse_databases_created(ch_utils)
            .await
            .expect("[verify_clickhouse]: Failed to check if all databases were created");

        if !dbs_created {
            return Err(EnvironmentError::from(
                "[verify_clickhouse]: Error: Databases were not created.",
            ));
        }

        self.dbg_print("[verify_clickhouse]: Check if clickhouse tables are already configured");
        let tables_created = self
            .verify_clickhouse_tables_created(ch_utils)
            .await
            .expect("[verify_clickhouse]: Failed to check if all database tables configured");

        if !tables_created {
            return Err(EnvironmentError::from(
                "[verify_clickhouse]: Error: Tables were not created.",
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

    /// Verifies if all databases are created in the ClickHouse database.
    ///
    /// This method checks if all databases required for the application are created in the ClickHouse database.
    ///
    /// # Arguments
    ///
    /// * `ch_utils` - A reference to a `ClickhouseUtil` object.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` if all databases exist.
    /// * `Ok(false)` if any database does not exist.
    /// * `Err(EnvironmentError)` if an error occurs during the verification process.
    ///
    async fn verify_clickhouse_databases_created(
        &self,
        ch_utils: &ClickhouseUtil,
    ) -> Result<bool, EnvironmentError> {
        self.dbg_print("[verify_databases_created]: Check if all databases exist");
        let all_exists = match ch_utils.verify_all_db_exists().await {
            Ok(exists) => exists,
            Err(e) => return Err(EnvironmentError::from(e.to_string())),
        };

        Ok(all_exists)
    }

    /// Verifies that all metadata tables exist in the ClickHouse database.
    ///
    /// This function performs the following steps:
    ///
    /// 1. Checks if all metadata tables exist in the ClickHouse database.
    ///
    /// # Arguments
    ///
    /// * `ch_utils` - A reference to a `ClickhouseUtil` object.
    ///
    /// # Returns
    ///
    /// - `Ok(true)` if all metadata tables exist.
    /// - `Ok(false)` if any metadata table does not exist.
    /// - `Err(EnvironmentError)` if an error occurs during the verification process.
    ///
    pub(crate) async fn verify_clickhouse_tables_created(
        &self,
        ch_utils: &ClickhouseUtil,
    ) -> Result<bool, EnvironmentError> {
        //
        self.dbg_print("[verify_tables_created]: Check if all metadata tables exist");
        let exists_metadata_tables = match ch_utils.metadata.verify_all_metadata_tables().await {
            Ok(exists) => exists,
            Err(e) => return Err(EnvironmentError::from(e.to_string())),
        };

        self.dbg_print("[verify_tables_created]: Check if all specs tables exist");
        let exists_specs_tables = match ch_utils.specs.verify_all_specs_tables().await {
            Ok(exists) => exists,
            Err(e) => return Err(EnvironmentError::from(e.to_string())),
        };

        let all_exists = exists_metadata_tables && exists_specs_tables;

        Ok(all_exists)
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

        let specs_data_imported = self
            .verify_specs_data_imported(ch_utils)
            .await
            .expect("[check_if_all_data_imported]: Failed to check if all specs data imported");

        let all_imported = metadata_imported && specs_data_imported;

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

    /// Asynchronously verifies if all specs data has been imported into the specs database.
    ///
    /// # Arguments
    ///
    /// * `ch_utils` - A reference to the ClickhouseUtil instance.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that indicates whether all specs data has been imported into the specs database.
    /// If successful, it returns `Ok(bool)` indicating whether all data has been imported.
    /// If an error occurs, it returns `Err(EnvironmentError)`.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `EnvironmentError`.
    ///
    async fn verify_specs_data_imported(
        &self,
        ch_utils: &ClickhouseUtil,
    ) -> Result<bool, EnvironmentError> {
        self.dbg_print(
            "[verify_specs_data_imported]: Check if all services data imported into the specs DB",
        );

        self.dbg_print("[verify_specs_data_imported]: Count services in DB");
        let nr_db_services = ch_utils
            .specs
            .count_services()
            .await
            .expect("[verify_specs_data_imported]: Failed to get count services from specs DB");

        self.dbg_print(&format!(
            "[verify_specs_data_imported]: Counted imported services: {}",
            nr_db_services
        ));

        let nr_service_specs = specs_utils::prelude::get_all_service_specs().len() as u64;
        self.dbg_print(&format!(
            "[verify_specs_data_imported]: Reference services: {}",
            nr_service_specs
        ));

        let services_imported = nr_service_specs == nr_db_services;
        self.dbg_print(&format!(
            "[verify_specs_data_imported]: All services imported: {}",
            services_imported
        ));

        let all_imported = services_imported;
        self.dbg_print(&format!(
            "[verify_specs_data_imported]: All specs data imported: {}",
            all_imported
        ));

        Ok(all_imported)
    }
}
