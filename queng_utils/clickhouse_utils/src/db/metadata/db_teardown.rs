use crate::db::metadata::{Metadata, DB_NAME};
use crate::types::error::ClickHouseUtilError;
use std::error::Error;

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

impl Metadata {
    pub(crate) async fn drop_metadata_db(&self) -> Result<(), Box<dyn Error>> {
        let ddl = format!("DROP DATABASE IF EXISTS {DB_NAME}");
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop metadata DB");

        Ok(())
    }
}
