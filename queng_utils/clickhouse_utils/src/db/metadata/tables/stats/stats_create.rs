use crate::db::metadata::{Metadata, DB_NAME};
use crate::error::ClickHouseUtilError;

impl Metadata {
    pub(crate) async fn create_stats_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.generate_create_stats_table_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        }
    }

    fn generate_create_stats_table_ddl(&self) -> String {
        format!(
            "
    CREATE TABLE IF NOT EXISTS {DB_NAME}.stats
    (
        `download_timestamp` String CODEC(LZ4),
        `hash` String CODEC(LZ4),
        `number_assets` UInt32 CODEC(Delta, LZ4),
        `number_exchanges` UInt32 CODEC(Delta, LZ4),
        `number_instruments` UInt32 CODEC(Delta, LZ4),
    )
    ENGINE = MergeTree
    PRIMARY KEY (download_timestamp)
    SETTINGS index_granularity = 10
    "
        )
    }
}
