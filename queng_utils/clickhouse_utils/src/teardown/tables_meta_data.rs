use crate::error::ClickHouseUtilError;
use crate::ClickhouseUtil;

impl ClickhouseUtil {
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

    async fn drop_stats_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.metadata.generate_drop_stats_table_ddl();
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop asset table");

        Ok(())
    }

    async fn drop_assets_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.metadata.generate_drop_asset_table_ddl();
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop asset table");

        Ok(())
    }

    async fn drop_exchanges_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.metadata.generate_drop_exchanges_table_ddl();
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop exchanges table");

        Ok(())
    }

    pub(crate) async fn drop_instruments_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.metadata.generate_drop_instruments_table_ddl();
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop instruments table");

        Ok(())
    }
}
