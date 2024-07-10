use crate::db::metadata::Metadata;
use crate::prelude::ClickHouseUtilError;

impl Metadata {
    pub async fn drop_metadata_tables(&self) -> Result<(), ClickHouseUtilError> {
        self.drop_stats_table()
            .await
            .expect("Failed to drop stats table");

        self.drop_assets_table()
            .await
            .expect("Failed to drop asset table");

        self.drop_exchanges_table()
            .await
            .expect("Failed to drop exchanges table");

        self.drop_instruments_table()
            .await
            .expect("Failed to drop instruments table");

        Ok(())
    }
}
