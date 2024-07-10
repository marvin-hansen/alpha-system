use crate::db::specs::{Specs, SERVICES_TABLE};
use crate::prelude::ClickHouseUtilError;

impl Specs {
    /// Asynchronously counts the number of services in the ClickHouse database.
    ///
    /// This method queries the `SERVICES_TABLE` table in the ClickHouse database and counts the number of rows.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that indicates the number of services in the ClickHouse database.
    /// If successful, it returns `Ok(u64)` containing the count of services.
    /// If an error occurs, it returns `Err(ClickHouseUtilError)`.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `ClickHouseUtilError`.
    ///
    pub async fn count_services(&self) -> Result<u64, ClickHouseUtilError> {
        return match self.count_rows(SERVICES_TABLE).await {
            Ok(count) => Ok(count),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        };
    }
}
