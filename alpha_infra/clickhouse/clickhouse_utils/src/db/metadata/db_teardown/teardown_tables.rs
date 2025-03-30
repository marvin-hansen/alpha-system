/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::ClickHouseUtilError;
use crate::db::metadata::Metadata;

impl Metadata {
    /// Drops all metadata tables in `ClickHouse`.
    ///
    /// This method is responsible for dropping all tables related to metadata in `ClickHouse`. It sequentially
    /// drops each metadata table including stats, assets, exchanges, and instruments tables.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all metadata tables are successfully dropped.
    /// If an error occurs during the dropping process, it returns an `Err(ClickHouseUtilError)`.
    ///
    /// # Errors
    ///
    /// This method can return any error that implements the `ClickHouseUtilError` trait.
    ///
    pub async fn drop_all_metadata_tables(&self) -> Result<(), ClickHouseUtilError> {
        self.drop_stats_table()
            .await
            .expect("Failed to drop stats table");

        self.drop_assets_table()
            .await
            .expect("Failed to drop asset table");

        self.drop_exchanges_table()
            .await
            .expect("Failed to drop exchanges table");

        self.drop_instruments_table()
            .await
            .expect("Failed to drop instruments table");

        Ok(())
    }
}
