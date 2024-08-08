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
        "INSERT INTO system.service VALUES({}, '{}', {}, {}, '{}', '{}', '{}', '{}',{})
            RETURNING id",
        data.svc_id().as_u8(),
        data.name(),
        data.version(),
        data.online(),
        data.description(),
        data.health_check_uri(),
        data.base_uri(),
        shared::service_ids_to_string(data.dependencies()),
        shared::service_endpoints_to_string(data.endpoints()),
    )
}
