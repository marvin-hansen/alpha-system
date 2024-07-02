use crate::db::metadata::{Metadata, DB_NAME};
use crate::types::error::ClickHouseUtilError;

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
            Ok(_) => Ok(()),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        }
    }

    fn generate_create_instruments_table_ddl(&self) -> String {
        format!(
            "
     CREATE TABLE IF NOT EXISTS {DB_NAME}.instruments
     (
       `trade_start_timestamp` UInt64 CODEC(Delta, LZ4),
       `trade_end_timestamp` UInt64 CODEC(Delta, LZ4),
       `exchange_code` StringWithDictionary CODEC(LZ4),
       `exchange_pair_code` StringWithDictionary CODEC(LZ4),
       `base_asset` StringWithDictionary CODEC(LZ4),
       `quote_asset` StringWithDictionary CODEC(LZ4),
       `code` StringWithDictionary CODEC(LZ4),
       `class` StringWithDictionary CODEC(LZ4),
       `pair_figi` String CODEC(LZ4),
       `instrument_figi` String CODEC(LZ4),
            PROJECTION projection_instruments_by_code
            (
                SELECT *
                GROUP BY
                trade_start_timestamp,
                trade_end_timestamp,
                    exchange_code,
                    exchange_pair_code,
                    base_asset,
                    quote_asset,
                    code,
                    class,
                    pair_figi,
                    instrument_figi
            )
     )
     ENGINE = MergeTree
     PRIMARY KEY (code, pair_figi)
     SETTINGS index_granularity = 2048
    "
        )
    }
}
