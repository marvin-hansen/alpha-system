use crate::error::PostgresDBError;
use crate::PostgresDBManager;
use common_config::prelude::{ServiceConfig, ServiceID};

// const SERVICE_TABLE: &str = "service";

impl PostgresDBManager {
    pub async fn insert_service(&self, _data: &ServiceConfig) -> Result<(), PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }

    pub async fn count_services(&self) -> Result<u64, PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }

    pub async fn check_if_service_id_exists(
        &self,
        _id: &ServiceID,
    ) -> Result<bool, PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }

    pub async fn check_if_services_exists(
        &self,
        services: &Vec<ServiceID>,
    ) -> Result<bool, PostgresDBError> {
        for id in services {
            if !self
                .check_if_service_id_exists(id)
                .await
                .expect("Failed to check if service id exists")
            {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub async fn check_if_service_id_online(
        &self,
        _id: &ServiceID,
    ) -> Result<bool, PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }

    pub async fn check_if_services_online(
        &self,
        services: &Vec<ServiceID>,
    ) -> Result<bool, PostgresDBError> {
        for id in services {
            if !self
                .check_if_service_id_online(id)
                .await
                .expect("Failed to check if service id exists")
            {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub async fn read_all_services(&self) -> Result<Vec<ServiceConfig>, PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }

    pub async fn read_record_by_id(
        &self,
        _id: &ServiceID,
    ) -> Result<Option<ServiceConfig>, PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }

    pub async fn set_service_online(&self, id: &ServiceID) -> Result<bool, PostgresDBError> {
        self.set_svc_online(id, true).await
    }

    pub async fn set_service_offline(&self, id: &ServiceID) -> Result<bool, PostgresDBError> {
        self.set_svc_online(id, false).await
    }

    async fn set_svc_online(
        &self,
        _id: &ServiceID,
        _online: bool,
    ) -> Result<bool, PostgresDBError> {
        // Test if service even exists
        // let exists = self
        //     .check_if_service_id_exists(id)
        //     .await
        //     .expect("Failed to check if service id exists");

        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }

    pub async fn update_service(
        &self,
        _data: ServiceConfig,
    ) -> Result<Option<ServiceConfig>, PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }

    pub async fn delete_service(&self, _id: &ServiceID) -> Result<bool, PostgresDBError> {
        Err(PostgresDBError::NotImplementedError(
            "Function not implemented".to_string(),
        ))
    }
}
