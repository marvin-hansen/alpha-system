use common_config::prelude::{ServiceConfig, ServiceID};

use crate::PostgresDBManager;

impl PostgresDBManager {
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

    pub(crate) fn build_read_all_services_query(&self) -> String {
        "SELECT
            id, name, version, online, description, health_check_uri, base_uri, dependencies, exposure, endpoint, metrics
         FROM
           system.service
         ORDER BY
            id
  ".to_string()
    }

    pub(crate) fn build_delete_service_query(&self, id: &ServiceID) -> String {
        format!(
            "DELETE FROM system.service
             WHERE
                id={}",
            id.as_u8()
        )
    }
}
