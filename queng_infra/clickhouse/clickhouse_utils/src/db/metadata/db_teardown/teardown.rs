/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::db::metadata::Metadata;
use crate::error::ClickHouseUtilError;

impl Metadata {
    /// Tears down the metadata database in `ClickHouse`.
    ///
    /// This method performs the teardown of the metadata database by executing the following steps:
    /// 1. Drops all metadata tables by calling the `drop_all_metadata_tables` method.
    /// 2. Optionally drops the metadata database if specified by the `drop_db` parameter.
    ///
    /// # Arguments
    ///
    /// - `drop_db`: A boolean flag indicating whether to drop the metadata database along with its tables.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the teardown process is successful.
    /// If an error occurs during the teardown, it returns an `Err(ClickHouseUtilError)`.
    ///
    /// # Errors
    ///
    /// This method can return a `ClickHouseUtilError` error.
    ///
    pub async fn teardown_metadata_db(&self, drop_db: bool) -> Result<(), ClickHouseUtilError> {
        self.dbg_print("[teardown]: drop_all_metadata_tables");
        match self.drop_all_metadata_tables().await {
            Ok(()) => (),
            Err(e) => return Err(e),
        }

        if drop_db {
            self.dbg_print("[teardown]: drop_metadata_db");
            match self.drop_metadata_db().await {
                Ok(()) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}
