use common_config::prelude::ServiceConfig;

use crate::shared;

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
        "UPDATE system.service
            SET
                name='{}',
                version={},
                online={},
                description='{}',
                health_check_uri='{}',
                base_uri='{}',
                dependencies='{}',
                endpoints='{}',
            WHERE
                id={}
            RETURNING service.online",
        data.name(),
        data.version(),
        data.online(),
        data.description(),
        data.health_check_uri(),
        data.base_uri(),
        shared::service_ids_to_string(data.dependencies()),
        shared::service_endpoints_to_string(data.endpoints()),
        data.svc_id().as_u8(),
    )
}
