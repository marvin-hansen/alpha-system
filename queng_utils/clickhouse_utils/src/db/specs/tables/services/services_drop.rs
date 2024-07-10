use crate::db::specs::{Specs, SERVICES_TABLE};
use crate::prelude::ClickHouseUtilError;

impl Specs {
    /// Asynchronously drops the services table in the ClickHouse database.
    ///
    /// This method generates the necessary DDL (Data Definition Language) statement
    /// to drop the `SERVICES_TABLE` table in the ClickHouse database.
    /// It then executes the DDL statement using the `execute_query` method.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of dropping the services table.
    /// If successful, it returns `Ok(())`.
    /// If an error occurs, it returns `Err(ClickHouseUtilError)`.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `ClickHouseUtilError`.
    ///
    pub(crate) async fn drop_services_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.generate_drop_table_ddl(SERVICES_TABLE);
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop services table");

        Ok(())
    }
}
