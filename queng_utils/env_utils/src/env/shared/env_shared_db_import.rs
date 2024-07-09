use crate::prelude::EnvironmentError;
use crate::EnvUtil;
use clickhouse_utils::ClickhouseUtil;
use kaiko_utils::KaikoUtil;

impl EnvUtil {
    /// Imports metadata from various sources into the environment.
    ///
    /// This method imports metadata from various sources into the environment. It takes references to a `ClickhouseUtil` object, a `KaikoUtil` object, and an optional `sample_size` argument.
    ///
    /// The method first retrieves the metadata from the Clickhouse database using the `get_metadata` method of the `ClickhouseUtil` object.
    ///
    /// Then, it retrieves the metadata from the Kaiko API using the `get_metadata` method of the `KaikoUtil` object.
    ///
    /// Finally, it merges the metadata from the Clickhouse database and the Kaiko API using the `merge_metadata` method.
    ///
    /// # Arguments
    ///
    /// * `ch_utils` - A reference to a `ClickhouseUtil` object.
    /// * `kaiko_util` - A reference to a `KaikoUtil` object.
    /// * `sample_size` - An optional `u32` value representing the sample size.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the metadata is imported successfully, or an `Err` variant of `EnvironmentError` if an error occurs during the import process.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `EnvironmentError` if any of the following errors occur during the import process:
    ///
    /// - `ClickhouseError`: If there is an error retrieving the metadata from the Clickhouse database.
    /// - `KaikoError`: If there is an error retrieving the metadata from the Kaiko API.
    ///
    pub(crate) async fn import_metadata(
        &self,
        ch_utils: &ClickhouseUtil,
        kaiko_util: &KaikoUtil,
        _sample_size: Option<u32>,
    ) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[import_metadata]: Download assets metadata");
        let assets = kaiko_util
            .get_assets()
            .await
            .expect("[import_metadata]: Failed to get assets");

        self.dbg_print(&format!(
            "[import_metadata]: Downloaded assets: {}",
            assets.len()
        ));

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

        self.dbg_print(&format!(
            "[import_metadata]: Downloaded exchanges: {}",
            exchanges.len()
        ));

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

        self.dbg_print(&format!(
            "[import_metadata]: Downloaded instruments: {}",
            instruments.len()
        ));

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
}
