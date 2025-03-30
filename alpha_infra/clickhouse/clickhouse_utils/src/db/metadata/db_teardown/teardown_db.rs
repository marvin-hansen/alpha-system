/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::db::metadata::{DB_NAME, Metadata};
use crate::error::ClickHouseUtilError;

impl Metadata {
    /// Drops the metadata database if it exists.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the database is successfully dropped.
    /// Returns `Err(ClickHouseUtilError)` if there is an error dropping the database.
    pub(crate) async fn drop_metadata_db(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = format!("DROP DATABASE IF EXISTS {DB_NAME}");

        match self.execute_query(&ddl).await {
            Ok(()) => (),
            Err(e) => return Err(ClickHouseUtilError::from(e.to_string())),
        };

        Ok(())
    }
}
