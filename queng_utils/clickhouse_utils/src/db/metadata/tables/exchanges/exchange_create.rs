use crate::db::metadata::{Metadata, DB_NAME};
use crate::types::error::ClickHouseUtilError;

impl Metadata {
    /// Creates the exchanges table in the metadata database if it does not already exist.
    ///
    /// This method creates the exchanges table in the metadata database if it does not already exist.
    /// It generates the SQL DDL statement for creating the table using the `generate_create_exchanges_table_ddl` method.
    /// The generated SQL statement is then executed using the `execute_query` method.
    ///
    /// # Arguments
    ///
    /// * `&self` - A reference to the `Metadata` object.
    ///
    /// # Returns
    ///
    /// * `Result<(), ClickHouseUtilError>` - The result of executing the query. Returns `Ok(())` if the table is created successfully, or an `Err` containing the error if the creation fails.
    ///
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
