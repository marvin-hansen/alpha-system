use crate::error::ClickHouseUtilError;
use crate::ClickhouseUtil;

impl ClickhouseUtil {
    pub async fn verify_all_metadata_tables(&self) -> Result<bool, ClickHouseUtilError> {
        self.metadata.verify_metadata_tables_created().await
    }

    pub async fn create_metadata_tables(&self) -> Result<(), ClickHouseUtilError> {
        //
        self.metadata
            .create_stats_table()
            .await
            .expect("[ClickhouseUtil]: Failed to create stats table");

        self.metadata
            .create_assets_table()
            .await
            .expect("[ClickhouseUtil]: Failed to create asset table");

        self.metadata
            .create_exchanges_table()
            .await
            .expect("[ClickhouseUtil]: Failed to create exchanges table");

        self.metadata
            .create_instruments_table()
            .await
            .expect("[ClickhouseUtil]: Failed to create instruments table");

        Ok(())
    }
}
