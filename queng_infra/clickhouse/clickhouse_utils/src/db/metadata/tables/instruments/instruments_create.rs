use crate::db::metadata::Metadata;
use crate::error::ClickHouseUtilError;

impl Metadata {
    /// Creates the instruments table in the metadata database if it does not already exist.
    ///
    /// This method creates the instruments table in the metadata database if it does not already exist.
    /// It generates the SQL DDL statement for creating the table using the `generate_create_instruments_table_ddl` method.
    /// The generated SQL statement is then executed using the `execute_query` method.
    ///
    /// # Returns
    ///
    /// * `Result<(), ClickHouseUtilError>` - The result of executing the query. Returns `Ok(())` if the table is created successfully, or an `Err` containing the error if the creation fails.
    ///
    pub(crate) async fn create_instruments_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.generate_create_instruments_table_ddl();
        match self.execute_query(&ddl).await {
            Ok(()) => Ok(()),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        }
    }
}
