use crate::types::TradeRow;
use crate::{ClickhouseDBManager, FN_NAME};
use common_database::prelude::sanitize_utils;
use futures::stream::BoxStream;
use futures::StreamExt;
use klickhouse::KlickhouseError;

impl ClickhouseDBManager {
    /// Stream trade bars for the given symbol from the database.
    ///
    /// This returns a stream of `TradeBar` structs for the specified `symbol_id`.
    /// Trade bars are fetched from the database and yielded as they become available.
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - The symbol ID to fetch trade bars for
    /// * `trade_table` - The name of the DB table to query and stream
    ///
    /// # Errors
    ///
    /// This function may return connection errors or other database errors.
    ///
    /// # Example
    ///
    pub async fn stream_trades<'a>(
        &'a self,
        trade_table: &'a str,
    ) -> BoxStream<Result<TradeRow, KlickhouseError>> {
        let sanitized_name =
            sanitize_utils::sanitize_table_name(trade_table).expect("Invalid table name");

        // Build the query
        let query = self.build_get_trades_query(sanitized_name);

        self.client
            .query::<TradeRow>(query)
            .await
            .unwrap_or_else(|_| panic!("{} Failed to execute stream_trades query ", FN_NAME))
            .boxed()
    }
}
