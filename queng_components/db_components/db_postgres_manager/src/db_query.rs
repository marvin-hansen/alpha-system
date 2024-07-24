use common_config::prelude::{ServiceConfig, ServiceID};

use crate::PostgresDBManager;

impl PostgresDBManager {
    /// Builds a PostgreSQL query to check if a service ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The service ID to check for existence.
    ///
    /// # Returns
    ///
    /// A PostgreSQL query string that checks if the service ID exists in the database.
    pub(crate) fn build_check_if_service_id_exists_query(&self, id: &ServiceID) -> String {
        format!(
            "
        SELECT EXISTS (
        SELECT
            id
        FROM
            system.service
        WHERE
            id={}
        )
        ",
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
    pub(crate) fn build_check_if_service_id_online_query(&self, id: &ServiceID) -> String {
        format!(
            "
        SELECT EXISTS (
        SELECT
            id, online
        FROM
            system.service
        WHERE
            id={}
        AND
            online=true
        )
        ",
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
    pub(crate) fn build_set_svc_online_query(&self, id: &ServiceID, online: bool) -> String {
        format!(
            "
        UPDATE
            system.service
        SET
            online={}
        WHERE
            id={}
        RETURNING service.online
        ",
            online,
            id.as_u8()
        )
    }

    /// Builds a PostgreSQL query to insert a service into the database.
    ///
    /// # Arguments
    ///
    /// * `data` - The service configuration to insert.
    ///
    /// # Returns
    ///
    /// A PostgreSQL query string that inserts the service into the database.
    pub(crate) fn build_insert_service_query(&self, data: &ServiceConfig) -> String {
        format!(
            "INSERT INTO system.service(id, name, version, online, description, health_check_uri, base_uri, dependencies, exposure, endpoint, metrics)
             VALUES({}, '{}', {}, {}, '{}', '{}', '{}', '{}', {},
                ROW('{}', {}, '{}', {}, {}),
                ROW('{}', '{}', {})
            )",
            data.svc_id().as_u8(), data.name(), data.version(), data.online(), data.description(), data.health_check_uri(), data.base_uri(), self.service_ids_to_string(data.dependencies()), data.exposure().as_u8(),
            data.endpoint().name(), data.endpoint().version(), data.endpoint().uri(), data.endpoint().port(), data.endpoint().protocol().as_u8(),
            data.metrics().uri(), data.metrics().host(), data.metrics().port()
        )
    }

    fn service_ids_to_string(&self, ids: &[ServiceID]) -> String {
        let id_strings: Vec<String> = ids.iter().map(|id| id.as_u8().to_string()).collect();
        format!("{{{}}}", id_strings.join(","))
    }

    /// Builds a PostgreSQL query to update a service in the database.
    ///
    /// # Arguments
    ///
    /// * `data` - The service configuration to update.
    ///
    /// # Returns
    ///
    /// A PostgreSQL query string that updates the service in the database.
    ///
    pub(crate) fn build_update_service_query(&self, data: &ServiceConfig) -> String {
        format!(
            "UPDATE system.service
             SET
                name = '{}',
                version = {},
                online = {},
                description = '{}',
                health_check_uri = '{}',
                base_uri = '{}',
                dependencies = '{}',
                exposure = {},
                endpoint = ROW('{}', {}, '{}', {}, {}),
                metrics = ROW('{}', '{}', {})
             WHERE
                id = {}
            ",
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
            data.metrics().port(),
            data.svc_id().as_u8()
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
    pub(crate) fn build_read_service_by_id_query(&self, id: &ServiceID) -> String {
        format!(
            "SELECT
                id, name, version, online, description, health_check_uri, base_uri, dependencies, exposure, endpoint, metrics
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
    pub(crate) fn build_read_all_services_query(&self) -> String {
        "SELECT
            id, name, version, online, description, health_check_uri, base_uri, dependencies, exposure, endpoint, metrics
         FROM
           system.service
         ORDER BY
            id
  ".to_string()
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
    pub(crate) fn build_delete_service_query(&self, id: &ServiceID) -> String {
        format!(
            "DELETE FROM system.service
             WHERE
                id={}",
            id.as_u8()
        )
    }
}
