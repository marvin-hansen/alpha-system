use crate::prelude::EnvironmentError;
use crate::EnvUtil;
use clickhouse_utils::ClickhouseUtil;
use kaiko_utils::KaikoUtil;

impl EnvUtil {
    /// Creates all tables required for testing in the ClickHouse database.
    ///
    /// This function creates all tables necessary for testing.
    ///
    /// # Arguments
    ///
    /// * `ch_utils` - A reference to a `ClickhouseUtil` object.
    ///
    /// # Errors
    ///
    /// - `EnvironmentError` if any step fails.
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
