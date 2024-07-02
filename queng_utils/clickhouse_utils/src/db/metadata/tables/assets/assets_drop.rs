use crate::db::metadata::{Metadata, DB_NAME};
use crate::types::error::ClickHouseUtilError;

impl Metadata {
    /// Drops the assets table in the metadata database, if it exists.
    ///
    /// This method drops the assets table in the metadata database, if it exists.
    /// It generates a SQL query to drop the table and executes it using the `execute_query` method.
    ///
    /// # Returns
    ///
    /// * `Result<(), ClickHouseUtilError>` - The result of executing the query. Returns `Ok(())` if the table is dropped successfully, or an `Err` containing the error if dropping fails.
    ///
    pub(crate) async fn drop_assets_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.generate_drop_asset_table_ddl();
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop asset table");

        Ok(())
    }

    fn generate_drop_asset_table_ddl(&self) -> String {
        format!("DROP TABLE IF EXISTS {DB_NAME}.assets")
    }
}
