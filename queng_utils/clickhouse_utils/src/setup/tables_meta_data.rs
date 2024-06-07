use crate::error::ClickHouseUtilError;
use crate::ClickhouseUtil;

impl ClickhouseUtil {
    pub async fn create_metadata_tables(&self) -> Result<(), ClickHouseUtilError> {
        //
        self.create_stats_table()
            .await
            .expect("[ClickhouseUtil]: Failed to create stats table");

        self.create_assets_table()
            .await
            .expect("[ClickhouseUtil]: Failed to create asset table");

        self.create_exchanges_table()
            .await
            .expect("[ClickhouseUtil]: Failed to create exchanges table");

        self.create_instruments_table()
            .await
            .expect("[ClickhouseUtil]: Failed to create instruments table");

        Ok(())
    }

    async fn create_stats_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.metadata.generate_create_stats_table_ddl();
        return match self.execute_query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        };
    }

    async fn create_assets_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.metadata.generate_create_asset_table_ddl();
        return match self.execute_query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        };
    }

    async fn create_exchanges_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.metadata.generate_create_exchanges_table_ddl();
        return match self.execute_query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        };
    }

    pub(crate) async fn create_instruments_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.metadata.generate_create_instruments_table_ddl();
        return match self.execute_query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        };
    }
}
