use crate::error::ClickHouseUtilError;
use crate::ClickhouseUtil;

impl ClickhouseUtil {
    pub async fn drop_metadata_tables(&self) -> Result<(), ClickHouseUtilError> {
        self.metadata
            .drop_stats_table()
            .await
            .expect("Failed to drop stats table");

        self.metadata
            .drop_assets_table()
            .await
            .expect("Failed to drop asset table");

        self.metadata
            .drop_exchanges_table()
            .await
            .expect("Failed to drop exchanges table");

        self.metadata
            .drop_instruments_table()
            .await
            .expect("Failed to drop instruments table");

        Ok(())
    }
}
