use crate::error::ClickHouseDBError;
use crate::types::OHLCVRow;
use crate::ClickhouseDBManager;
use common_data_bar::prelude::{OHLCVBar, TimeResolution};
use common_database::prelude::sanitize_utils;

impl ClickhouseDBManager {
    /// Retrieves all OHLCV data bars for the given symbol table and time resolution.
    ///
    /// # Parameters
    ///
    /// - `symbol_table` - The name of the symbol table to query
    /// - `time_resolution` - The time resolution to use for the query
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<DataBar>)` - The vector containing all the OHLCV data bars.
    /// - `Err(QueryError)` - If there was an error executing the query.
    ///
    /// # Errors
    ///
    /// - Returns a `QueryError` if:
    ///   - The table name could not be sanitized
    ///   - The query failed to execute
    ///
    /// # Remarks
    ///
    /// - Sanitizes the table name to prevent SQL injection.
    /// - Builds a SQL query based on the parameters.
    /// - Executes the query and converts the rows to `DataBar` objects.
    /// - Returns an empty vector if there are no results.
    ///
    pub async fn get_all_ohlcv_bars(
        &mut self,
        symbol_id: u16,
        symbol_table: &str,
        time_resolution: &TimeResolution,
    ) -> Result<Vec<OHLCVBar>, ClickHouseDBError> {
        // Sanitize table name input to prevent SQL injection.

        let sanitized_name = match sanitize_utils::sanitize_table_name(symbol_table) {
            Ok(name) => name,
            Err(e) => return Err(ClickHouseDBError::TableSanitizeError(e.to_string())),
        };

        // Build the query
        let query = self.build_get_ohlcv_bars_query(sanitized_name, time_resolution);

        // Execute query
        let ohlcv_rows = match self.client.query_collect::<OHLCVRow>(&query).await {
            Ok(res) => res,
            Err(e) => return Err(ClickHouseDBError::QueryFailed(e.to_string())),
        };

        // Check for empty result
        if ohlcv_rows.is_empty() {
            return Ok(Vec::new());
        }

        let mut bars = Vec::with_capacity(ohlcv_rows.len());

        for row in ohlcv_rows {
            let bar = OHLCVBar::new(
                symbol_id,
                row.date_time(),
                row.open(),
                row.high(),
                row.low(),
                row.close(),
                row.volume(),
            );
            bars.push(bar);
        }

        Ok(bars)
    }
}
