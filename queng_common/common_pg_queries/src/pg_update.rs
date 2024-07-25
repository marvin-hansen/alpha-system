use common_config::prelude::ServiceConfig;

use crate::shared::service_ids_to_string;

/// Builds a PostgreSQL query string that updates a service in the database.
///
/// # Args
///
/// * `data` - The service configuration to update.
///
/// # Returns
///
/// A PostgreSQL query string that updates the service in the database.
///
pub fn build_update_service_query(data: &ServiceConfig) -> String {
    format!(
        "
            UPDATE
                system.service
            SET
                name='{}', version={}, online={}, description='{}', health_check_uri='{}',base_uri='{}',  dependencies='{}', exposure={},
                endpoint_name='{}', endpoint_version={}, endpoint_base_uri='{}',  endpoint_port={}, endpoint_protocol={},
                metric_uri='{}', metric_host='{}', metric_port={}
            WHERE
                id={}
            RETURNING service.online
            ",
        data.name(),
        data.version(),
        data.online(),
        data.description(),
        data.health_check_uri(),
        data.base_uri(),
        service_ids_to_string(data.dependencies()),
        data.exposure().as_u8(),
        data.endpoint().name(),
        data.endpoint().version(),
        data.endpoint().uri(),
        data.endpoint().port(),
        data.endpoint().protocol().as_u8(),
        data.metrics().uri(),
        data.metrics().host(),
        data.metrics().port(),
        data.svc_id().as_u8(),
    )
}
