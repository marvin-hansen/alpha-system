use crate::db::metadata::{Metadata, STATS_TABLE};
use crate::types::error::ClickHouseUtilError;

impl Metadata {
    /// Drops the stats table in the metadata database if it exists.
    ///
    /// This method drops the stats table in the metadata database if it exists.
    /// It generates a SQL query to drop the table using the `generate_drop_stats_table_ddl` method.
    /// The generated query is then executed using the `execute_query` method.
    ///
    /// # Returns
    ///
    /// * `Result<(), ClickHouseUtilError>` - The result of executing the query. Returns `Ok(())` if the table is dropped successfully, or an `Err` containing the error if dropping fails.
    ///
    pub(crate) async fn drop_stats_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.generate_drop_table_ddl(STATS_TABLE);

        self.execute_query(&ddl)
            .await
            .expect("Failed to drop asset table");

        Ok(())
    }
}
