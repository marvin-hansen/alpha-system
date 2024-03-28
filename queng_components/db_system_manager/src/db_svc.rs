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

        // check if id is in service_cache, if so return
        if self.service_cache.read().unwrap().contains_key(id) {
            return Ok(true);
        }

        // Check if id is in the database, if so add it to the cache and return

        // Otherwise, return false
        Ok(false)
    }

    pub async fn check_if_services_exists(&self, services: &Vec<ServiceID>) -> Result<bool, Error> {
        for id in services {
            if !self.check_if_service_id_exists(id).await? {
                return Ok(false);
            }
        }

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
        // Check if the cache is empty, if so query the database, add to cache and return.
        if self.service_cache.read().unwrap().is_empty() {
            // TODO
        }

        // otherwise return service_cache as vector
        let services = self
            .service_cache
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<ServiceConfig>>();

        Ok(services)
    }

    pub async fn read_record_by_id(&self, id: &ServiceID) -> Result<Option<ServiceConfig>, Error> {
        // Check if id is in service_cache, if so return,
        if self.service_cache.read().unwrap().contains_key(id) {
            let service = self.service_cache.read().unwrap().get(id).unwrap().clone();
            return Ok(Some(service));
        }

        // Otherwise, query the database and add it to the cache and return

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

        // check if id is in service_cache, if so remove it from the cache
        if self.service_cache.read().unwrap().contains_key(id) {
            // self.service_cache.remove(id);
        }

        // Otherwise, check if id is in database, if so delete from database.

        Ok(true)
    }
}
