use crate::PostgresSMDBManager;
use common_config::prelude::{ServiceConfig, ServiceID};
use common_errors::prelude::PostgresDBError;
use pg_smdb::model::service;

impl PostgresSMDBManager {
    pub async fn insert_service(
        &mut self,
        service_config: &ServiceConfig,
    ) -> Result<(), PostgresDBError> {
        self.dbg_print("insert_service");
        let conn = &mut self.conn;

        match service::Service::create(conn, &service_config) {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    pub async fn count_services(&mut self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_services");
        let conn = &mut self.conn;

        match service::Service::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
        }
    }

    pub async fn check_if_service_id_exists(
        &mut self,
        id: &ServiceID,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_service_id_exists");
        let conn = &mut self.conn;
        match service::Service::check_if_service_id_exists(conn, *id) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        }
    }

    pub async fn check_if_services_exists(
        &mut self,
        services: &Vec<ServiceID>,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_services_exists");

        for id in services {
            match self.check_if_service_id_exists(id).await {
                Ok(exists) => {
                    if !exists {
                        return Ok(false);
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(true)
    }

    pub async fn check_if_service_id_online(
        &mut self,
        id: &ServiceID,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_service_id_online");
        let conn = &mut self.conn;

        match service::Service::check_if_service_id_online(conn, *id) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        }
    }

    pub async fn check_if_services_online(
        &mut self,
        services: &Vec<ServiceID>,
    ) -> Result<bool, PostgresDBError> {
        for id in services {
            match self.check_if_service_id_online(id).await {
                Ok(exists) => {
                    if !exists {
                        return Ok(false);
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(true)
    }

    pub async fn set_service_online(&mut self, id: &ServiceID) -> Result<(), PostgresDBError> {
        self.dbg_print("set_service_online");
        let conn = &mut self.conn;
        match service::Service::set_service_online(conn, *id) {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresDBError::SetFieldFailed(e.to_string())),
        }
    }

    pub async fn set_service_offline(&mut self, id: &ServiceID) -> Result<(), PostgresDBError> {
        self.dbg_print("set_service_offline");
        let conn = &mut self.conn;

        match service::Service::set_service_online(conn, *id) {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresDBError::SetFieldFailed(e.to_string())),
        }
    }

    pub async fn read_service_by_id(
        &mut self,
        id: &ServiceID,
    ) -> Result<Option<ServiceConfig>, PostgresDBError> {
        self.dbg_print("read_service_by_id");

        let conn = &mut self.conn;
        match service::Service::read(conn, *id) {
            Ok(svc) => Ok(Some(svc)),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn read_all_services(&mut self) -> Result<Vec<ServiceConfig>, PostgresDBError> {
        self.dbg_print("read_all_services");

        let conn = &mut self.conn;
        match service::Service::read_all(conn) {
            Ok(services) => Ok(services),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn update_service(
        &mut self,
        data: ServiceConfig,
    ) -> Result<Option<ServiceConfig>, PostgresDBError> {
        self.dbg_print("update_service");

        let conn = &mut self.conn;

        match service::Service::update(conn, data.svc_id(), &data) {
            Ok(svc) => Ok(Some(svc)),
            Err(e) => Err(PostgresDBError::UpdateFailed(e.to_string())),
        }
    }
    pub async fn delete_service(&mut self, id: &ServiceID) -> Result<bool, PostgresDBError> {
        self.dbg_print("delete_service");
        let conn = &mut self.conn;

        match service::Service::delete(conn, *id) {
            Ok(_) => Ok(true),
            Err(e) => Err(PostgresDBError::DeleteFailed(e.to_string())),
        }
    }
}
