use common_config::prelude::ServiceID;

pub fn generate_count_table_query(schema_name: &str, table_name: &str) -> String {
    format!("SELECT COUNT(*) FROM {schema_name}.{table_name};")
}

/// Builds a PostgreSQL query to check if a service ID exists in the database.
///
/// # Arguments
///
/// * `id` - The service ID to check for existence.
///
/// # Returns
///
/// A PostgreSQL query string that checks if the service ID exists in the database.
pub fn build_check_if_service_id_exists_query(id: &ServiceID) -> String {
    format!(
        "SELECT EXISTS (
        SELECT
            id
        FROM
            system.service
        WHERE
            id={}
        )",
        id.as_u8()
    )
}

/// Builds a PostgreSQL query to check if a service ID is online in the database.
///
/// # Arguments
///
/// * `id` - The service ID to check for online status.
///
/// # Returns
///
/// A PostgreSQL query string that checks if the service ID is online in the database.
pub fn build_check_if_service_id_online_query(id: &ServiceID) -> String {
    format!(
        "SELECT EXISTS (
        SELECT
            id, online
        FROM
            system.service
        WHERE
            id={}
        AND
            online=true
        )",
        id.as_u8()
    )
}

/// Builds a PostgreSQL query to set the online status of a service in the database.
///
/// # Arguments
///
/// * `id` - The service ID to set the online status for.
/// * `online` - The online status to set.
///
/// # Returns
///
/// A PostgreSQL query string that sets the online status of the service in the database.
pub fn build_set_svc_online_query(id: &ServiceID, online: bool) -> String {
    format!(
        "UPDATE
            system.service
        SET
            online={}
        WHERE
            id={}
        RETURNING service.online",
        online,
        id.as_u8()
    )
}

/// Builds a PostgreSQL query to select a service by ID from the database.
///
/// # Arguments
///
/// * `id` - The service ID to select.
///
/// # Returns
///
/// A PostgreSQL query string that selects the service by ID from the database.
pub fn build_read_service_by_id_query(id: &ServiceID) -> String {
    format!(
        "SELECT
                id, name, version, online, description, health_check_uri, base_uri, dependencies, exposure,
                endpoint_name, endpoint_version, endpoint_base_uri, endpoint_port, endpoint_protocol,
                metric_uri, metric_host, metric_port
            FROM
                system.service
            WHERE
                id={}",
        id.as_u8()
    )
}

/// Builds a PostgreSQL query to select all services from the database.
///
/// # Returns
///
/// A PostgreSQL query string that selects all services from the database.
pub fn build_read_all_services_query() -> String {
    "SELECT
             id, name, version, online, description, health_check_uri, base_uri, dependencies, exposure,
             endpoint_name, endpoint_version, endpoint_base_uri, endpoint_port, endpoint_protocol,
             metric_uri, metric_host, metric_port
         FROM
           system.service
         ORDER BY
            id".to_string()
}

/// Builds a PostgreSQL query to delete a service by ID from the database.
///
/// # Arguments
///
/// * `id` - The service ID to delete.
///
/// # Returns
///
/// A PostgreSQL query string that deletes the service by ID from the database.
pub fn build_delete_service_query(id: &ServiceID) -> String {
    format!(
        "DELETE FROM system.service
             WHERE
                id={}",
        id.as_u8()
    )
}
