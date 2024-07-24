use common_config::prelude::{ServiceConfig, ServiceID};

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

        let query = self.build_insert_service_query(data);
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

    /// Builds the SQL query for inserting a service into the system.service table.
    ///
    /// This method takes a `ServiceConfig` object and generates an SQL query string
    /// to insert a new service into the system.service table. The query includes all
    /// the fields of the `ServiceConfig` object, including the service ID, name,
    /// version, online status, description, health check URI, base URI, dependencies,
    /// exposure level, endpoint, and metrics.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to a `ServiceConfig` object from which a SQL query is generated.
    ///
    /// # Returns
    ///
    /// Returns a `String` containing the SQL query for inserting the service.
    ///
    fn build_insert_service_query(&self, data: &ServiceConfig) -> String {
        format!(
            "INSERT INTO system.service(id, name, version, online, description, health_check_uri,
            base_uri, dependencies, exposure,
            endpoint_name, endpoint_version, endpoint_base_uri, endpoint_port, endpoint_protocol,
            metric_uri, metric_host, metric_port)
             VALUES({}, '{}', {}, {}, '{}', '{}', '{}', '{}', {},
                '{}', {}, '{}', {}, {},
                '{}', '{}', {}
            )",
            data.svc_id().as_u8(),
            data.name(),
            data.version(),
            data.online(),
            data.description(),
            data.health_check_uri(),
            data.base_uri(),
            self.service_ids_to_string(data.dependencies()),
            data.exposure().as_u8(),
            data.endpoint().name(),
            data.endpoint().version(),
            data.endpoint().uri(),
            data.endpoint().port(),
            data.endpoint().protocol().as_u8(),
            data.metrics().uri(),
            data.metrics().host(),
            data.metrics().port()
        )
    }

    fn service_ids_to_string(&self, ids: &[ServiceID]) -> String {
        let id_strings: Vec<String> = ids.iter().map(|id| id.as_u8().to_string()).collect();
        format!("{{{}}}", id_strings.join(","))
    }
}
