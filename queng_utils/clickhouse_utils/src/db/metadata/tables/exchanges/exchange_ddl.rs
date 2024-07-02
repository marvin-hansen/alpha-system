use crate::db::metadata::{Metadata, DB_NAME};
use crate::error::ClickHouseUtilError;

impl Metadata {
    pub(crate) async fn create_exchanges_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.generate_create_exchanges_table_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        }
    }

    pub(crate) fn generate_create_exchanges_table_ddl(&self) -> String {
        format!(
            "
     CREATE TABLE IF NOT EXISTS {DB_NAME}.exchanges
     (
       `code` String CODEC(LZ4),
       `name` String CODEC(LZ4),

        PROJECTION projection_exchanges_by_code
        (
            SELECT *
            GROUP BY
                code,
                name
        )
     )
    ENGINE = MergeTree
    PRIMARY KEY (code, name)
    SETTINGS index_granularity = 1024
    "
        )
    }
}

impl Metadata {
    pub(crate) fn generate_drop_exchanges_table_ddl(&self) -> String {
        format!("DROP TABLE IF EXISTS {DB_NAME}.exchanges")
    }
}
