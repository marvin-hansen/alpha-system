/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::types::OHLCVRow;
use crate::{ClickhouseDBManager, FN_NAME};
use common_data_bar::TimeResolution;
use common_database::sanitize_utils;
use futures::stream::BoxStream;
use futures::StreamExt;
use klickhouse::KlickhouseError;

impl ClickhouseDBManager {
    pub async fn stream_ohlcv(
        &self,
        symbol_table: &str,
        time_resolution: &TimeResolution,
    ) -> BoxStream<Result<OHLCVRow, KlickhouseError>> {
        // Sanitize table name input to prevent SQL injection.
        let sanitized_name = sanitize_utils::sanitize_table_name(symbol_table)
            .expect("Failed to sanitize table name");

        // Build the query
        let query = self.build_get_ohlcv_bars_query(sanitized_name, time_resolution);

        // Return the stream of rows
        self.client
            .query::<OHLCVRow>(query)
            .await
            .unwrap_or_else(|_| panic!("{FN_NAME} Failed to execute stream_ohlcv query "))
            .boxed()
    }
}
