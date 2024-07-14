use crate::types::OHLCVRow;
use crate::{ClickhouseDBManager, FN_NAME};
use clickhouse_utils::query_utils::sanitize_table_name;
use common_data_bar::prelude::TimeResolution;
use futures::stream::BoxStream;
use futures::StreamExt;
use klickhouse::KlickhouseError;

impl ClickhouseDBManager {
    pub async fn stream_ohlcv<'a>(
        &'a self,
        symbol_table: &str,
        time_resolution: &TimeResolution,
    ) -> BoxStream<Result<OHLCVRow, KlickhouseError>> {
        // Sanitize table name input to prevent SQL injection.
        let sanitized_name =
            sanitize_table_name(symbol_table).expect("Failed to sanitize table name");

        // Build the query
        let query = self.build_get_ohlcv_bars_query(sanitized_name, time_resolution);

        // Return the stream of rows
        self.client
            .query::<OHLCVRow>(query)
            .await
            .unwrap_or_else(|_| panic!("{} Failed to execute stream_ohlcv query ", FN_NAME))
            .boxed()
    }
}
