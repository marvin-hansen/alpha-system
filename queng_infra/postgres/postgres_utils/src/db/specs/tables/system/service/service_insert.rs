use common_config::prelude::ServiceConfig;
use common_pg_queries::service_insert;

use crate::db::Specs;
use crate::prelude::PostgresUtilError;

// insert into service(id,name,version,online,description,health_check_uri,base_uri,dependencies,exposure,endpoint,metrics )
// VALUES(
// 1,
// 'test',
// 1,
// false,
// 'test_description',
// 'test_health_check_uri',
// 'test_base_uri',
// '{1,2,3}',
// 42,
// ROW('test_endpoint', 1, '/', 7070, 3),
// ROW('/metrics', 'localhost', 8080)
// )
// RETURNING *;

impl Specs {
    /// Inserts a service into the system.service table.
    ///
    /// This method takes a `ServiceConfig` object and inserts a new service into the
    /// system.service table. The method generates an SQL query using the `build_insert_query`
    /// method and executes it using the `execute_query` method. If the query is successful,
    /// the method returns `Ok(())`. Otherwise, it returns an `Err` containing a
    /// `PostgresUtilError` with a message indicating the failure.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to a `ServiceConfig` object containing the data for the
    ///   new service.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the service is successfully inserted,
    /// or an `Err` containing a `PostgresUtilError` with a message indicating the failure.
    ///
    pub async fn insert_service(&self, data: &ServiceConfig) -> Result<(), PostgresUtilError> {
        self.dbg_print("insert_service");

        let query = service_insert::build_insert_service_query(data);
        // println!("query: {}", query);
        match self.execute_query(&query).await {
            Ok(_) => Ok(()),
            Err(err) => Err(PostgresUtilError::new(format!(
                "Failed to insert service: {} due error: {}",
                &data.name(),
                err
            ))),
        }
    }
}
