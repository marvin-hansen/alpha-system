use crate::db::specs::{Specs, DB_NAME, SERVICES_TABLE};
use crate::prelude::ClickHouseUtilError;

impl Specs {
    /// Asynchronously creates the services table in the ClickHouse database.
    ///
    /// This method generates the necessary DDL (Data Definition Language) statement to create the `SERVICES_TABLE` table in the ClickHouse database.
    /// It then executes the DDL statement using the `execute_query` method of the `Specs` struct.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of creating the services table.
    /// If successful, it returns `Ok(())`.
    /// If an error occurs, it returns `Err(ClickHouseUtilError)`.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `ClickHouseUtilError`.
    ///
    pub(crate) async fn create_service_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.generate_create_service_table_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        }
    }

    fn generate_create_service_table_ddl(&self) -> String {
        format!("CREATE TABLE IF NOT EXISTS {DB_NAME}.{SERVICES_TABLE} (
            svc_id UInt8,
            svc_name String,
            svc_version UInt8,
            online Boolean,
            description String,
            health_check_uri String,
            base_uri String,
            dependencies Array(UInt8),
            exposure UInt8,
            endpoint Tuple (endpoint_name String, endpoint_version UInt8, uri String, port UInt16, protocol UInt8),
            metrics Tuple (uri String, host String, port UInt16),
    )
    ENGINE = MergeTree()
    PRIMARY KEY (svc_id)
    SETTINGS index_granularity = 5
    ;
    "
        )
    }
}
