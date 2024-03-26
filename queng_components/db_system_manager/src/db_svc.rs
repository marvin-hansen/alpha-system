use crate::SystemDBManager;
use common::prelude::{ServiceConfig, ServiceID};
use std::fmt::Error;

impl SystemDBManager {
    pub async fn create_service(&self, data: ServiceConfig) -> Result<bool, Error> {
        println!("{}", data);

        Ok(true)
    }

    pub async fn check_if_service_id_exists(&self, id: &ServiceID) -> Result<bool, Error> {
        println!("{}", id);

        Ok(true)
    }

    pub async fn check_if_services_exists(&self, services: &Vec<ServiceID>) -> Result<bool, Error> {
        println!("{}", services.len());

        Ok(true)
    }

    pub async fn check_if_service_id_online(&self, id: &ServiceID) -> Result<bool, Error> {
        println!("{}", id);

        Ok(true)
    }

    pub async fn check_if_services_online(&self, services: &Vec<ServiceID>) -> Result<bool, Error> {
        println!("{}", services.len());

        Ok(true)
    }

    pub async fn read_all_services(&self) -> Result<Vec<ServiceConfig>, Error> {
        Ok(vec![])
    }

    pub async fn read_record_by_id(&self, id: &ServiceID) -> Result<Option<ServiceConfig>, Error> {
        println!("{}", id);

        Ok(None)
    }

    pub async fn set_service_online(&self, id: &ServiceID) -> Result<bool, Error> {
        println!("{}", id);

        Ok(true)
    }

    pub async fn set_service_offline(&self, id: &ServiceID) -> Result<bool, Error> {
        println!("{}", id);

        Ok(true)
    }

    pub async fn update_service(
        &self,
        data: ServiceConfig,
    ) -> Result<Option<ServiceConfig>, Error> {
        println!("{}", data);

        Ok(None)
    }

    pub async fn delete_service(&self, id: &ServiceID) -> Result<bool, Error> {
        println!("{}", id);

        Ok(true)
    }
}
