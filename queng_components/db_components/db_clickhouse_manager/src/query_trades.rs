use crate::error::ClickHouseDBError;
use crate::types::TradeRow;
use crate::ClickhouseDBManager;
use common_data_bar::prelude::TradeBar;
use common_database::prelude::sanitize_utils;

impl ClickhouseDBManager {
    /// Retrieves all trade bars for the given symbol table from the database.
    ///
    /// # Arguments
    ///
    /// * `symbol_table` - The name of the symbol table to query
    ///
    /// # Returns
    ///
    /// A `Result` with a `Vec` of `TradeBar` structs if successful, or a `QueryError` if an error occurs.
    ///
    /// # Errors
    ///
    /// This function may return the following errors:
    ///
    /// - `QueryError::QueryFailed` if the query to the DB failed.
    /// - `QueryError::EmptyTableName` if `table_name` is empty
    /// - `QueryError::InvalidTableName` if `table_name` contains invalid characters
    /// - `QueryError::TableNameTooLong` if `table_name` is longer than 64 characters
    ///
    /// See wrapped errors for more details.
    ///
    pub async fn get_all_trades(
        &mut self,
        symbol_id: u16,
        symbol_table: &str,
    ) -> Result<Vec<TradeBar>, ClickHouseDBError> {
        // Sanitize table name input to prevent SQL injection.
        let sanitized_name = match sanitize_utils::sanitize_table_name(symbol_table) {
            Ok(name) => name,
            Err(e) => return Err(ClickHouseDBError::TableSanitizeError(e.to_string())),
        };

        // Build the query
        let query = self.build_get_trades_query(sanitized_name);

        // Execute query
        let trade_rows = match self.client.query_collect::<TradeRow>(&query).await {
            Ok(res) => res,
            Err(e) => return Err(ClickHouseDBError::QueryFailed(e.to_string())),
        };

        // Check for empty result
        if trade_rows.is_empty() {
            return Ok(Vec::new());
        }

        let mut trades = Vec::with_capacity(trade_rows.len());

        for row in trade_rows {
            let bar = TradeBar::new(symbol_id, row.date_time(), row.price(), row.volume());
            trades.push(bar);
        }

        Ok(trades)
    }
}
