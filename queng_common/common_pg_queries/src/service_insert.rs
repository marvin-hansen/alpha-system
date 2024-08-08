use common_config::prelude::ServiceConfig;

use crate::shared;

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
pub fn build_insert_service_query(data: &ServiceConfig) -> String {
    format!(
        "INSERT INTO system.service(id, name, version, online, description, health_check_uri,
            base_uri, dependencies,
            endpoint_name, endpoint_version, endpoint_base_uri, endpoint_port, endpoint_protocol,
            metric_uri, metric_host, metric_port)
             VALUES({}, '{}', {}, {}, '{}', '{}', '{}', '{}', {},
                '{}', {}, '{}', {}, {},
                '{}', '{}'
            )
            RETURNING id",
        data.svc_id().as_u8(),
        data.name(),
        data.version(),
        data.online(),
        data.description(),
        data.health_check_uri(),
        data.base_uri(),
        shared::service_ids_to_string(data.dependencies()),
        data.service_endpoint().name(),
        data.service_endpoint().version(),
        data.service_endpoint().uri(),
        data.service_endpoint().port(),
        data.service_endpoint().protocol().as_u8(),
        data.metrics_endpoint().uri(),
        data.metrics_endpoint().host(),
        data.metrics_endpoint().port()
    )
}
