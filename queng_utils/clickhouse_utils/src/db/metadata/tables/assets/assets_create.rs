use crate::db::metadata::{Metadata, DB_NAME};
use crate::types::error::ClickHouseUtilError;

impl Metadata {
    /// Creates the assets table in the metadata database.
    ///
    /// This method creates the assets table in the metadata database. It generates a SQL query to create the table and executes it using the `execute_query` method.
    ///
    /// # Returns
    ///
    /// * `Result<(), ClickHouseUtilError>` - The result of executing the query. Returns `Ok(())` if the table is created successfully, or an `Err` containing the error if creation fails.
    ///
    pub(crate) async fn create_assets_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.generate_create_asset_table_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        }
    }

    fn generate_create_asset_table_ddl(&self) -> String {
        format!(
            "
    CREATE TABLE IF NOT EXISTS {DB_NAME}.assets
    (
        `code` String CODEC(LZ4),
        `name` String CODEC(LZ4),
        `asset_class` LowCardinality(String) CODEC(LZ4),
        `asset_figi` String CODEC(LZ4),

        PROJECTION projection_assets_by_class
        (
            SELECT *
            GROUP BY
                code,
                name,
                asset_class,
                asset_figi
        )
    )
    ENGINE = MergeTree
    PRIMARY KEY (code, name, asset_figi)
    SETTINGS index_granularity = 1024
    "
        )
    }
}
