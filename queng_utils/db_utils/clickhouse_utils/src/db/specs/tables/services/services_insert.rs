use crate::db::specs::{Specs, DB_NAME, SERVICES_TABLE};
use crate::prelude::ClickHouseUtilError;
use common_config::prelude::{Endpoint, MetricConfig, ServiceConfig, ServiceID};

impl Specs {
    /// Asynchronously imports service configurations into the services table in the ClickHouse database.
    ///
    /// This method takes a vector of `ServiceConfig` structs and iterates through each service configuration.
    /// For each service, it generates an insert query using the `generate_service_insert` method
    /// and executes the query using the `execute_query` method.
    ///
    /// # Arguments
    ///
    /// * `services` - A reference to a vector of `ServiceConfig` structs.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of importing the service configurations.
    /// If successful, it returns `Ok(())`.
    /// If an error occurs during the import process, it returns `Err(ClickHouseUtilError)`.
    ///
    /// # Errors
    ///
    /// This method can return an error of type `ClickHouseUtilError`.
    ///
    pub async fn import_service_specs(
        &self,
        services: &[ServiceConfig],
    ) -> Result<(), ClickHouseUtilError> {
        for service in services.iter() {
            let insert_query = self.generate_service_insert(service);

            self.execute_query(&insert_query)
                .await
                .expect("Failed to insert asset")
        }

        Ok(())
    }

    pub fn generate_service_insert(&self, svc: &ServiceConfig) -> String {
        let table_name = format!("{DB_NAME}.{SERVICES_TABLE}");
        let svc_id = svc.svc_id().as_u8();
        let name = svc.name().to_string();
        let version = svc.version();
        let online = svc.online();
        let description = svc.description();
        let health_check_uri = svc.health_check_uri();
        let base_uri = svc.base_uri();
        let exposure = svc.exposure().as_u8();
        let deps = self.get_dependencies_array(svc.dependencies());
        let endpoint = self.get_endpoint_tuple(&svc.endpoint());
        let metrics = self.get_metric_config_tuple(svc.metrics());

        format!(
            r"INSERT INTO {table_name} (*)
            VALUES (
            {svc_id},
            '{name}',
            {version},
            {online},
            '{description}',
            '{health_check_uri}',
            '{base_uri}',
            {deps},
            {exposure},
            {endpoint},
            {metrics}
            );"
        )
    }

    fn get_dependencies_array(&self, dependencies: &[ServiceID]) -> String {
        let mut s = String::new();
        for d in dependencies.iter() {
            let svc_dep = format!("{}", d.as_u8());
            s.push_str(&svc_dep);
            s.push(',');
        }

        format!("[{s}]")
    }

    // https://chistadata.com/knowledge-base/tuples-in-clickhouse/
    fn get_endpoint_tuple(&self, endpoint: &Endpoint) -> String {
        let name = endpoint.name();
        let version = endpoint.version();
        let uri = endpoint.uri();
        let port = endpoint.port();
        let protocol = endpoint.protocol().as_u8();

        format!(r"('{name}',{version},'{uri}',{port},{protocol})")
    }

    fn get_metric_config_tuple(&self, metrics: &MetricConfig) -> String {
        let uri = metrics.uri();
        let host = metrics.host();
        let port = metrics.port();

        format!(r"('{uri}','{host}',{port})")
    }
}
