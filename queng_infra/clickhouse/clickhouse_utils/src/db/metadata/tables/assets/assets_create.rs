use crate::db::metadata::Metadata;
use crate::error::ClickHouseUtilError;

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
            Ok(()) => Ok(()),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        }
    }
}
