use surrealdb::Error;
use surrealdb::opt::PatchOp;

use common::prelude::{ServiceConfig, ServiceID};

use crate::prelude::DBManager;

const SERVICE_TABLE: &str = "service";

impl DBManager {
    pub async fn create_service(&self, data: ServiceConfig) -> Result<bool, Error> {
        let table = SERVICE_TABLE;
        let id = data.svc_id().to_string();

        let created: Option<ServiceConfig> = self
            .db
            .update((table, id))
            .merge(data)
            .await
            .expect("Failed to create service");

        match created {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    pub async fn check_if_service_id_exists(&self, id: &ServiceID) -> Result<bool, Error> {
        let res = self
            .read_record_by_id(id)
            .await
            .expect("Failed to check if service id exists");

        match res {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    pub async fn check_if_services_exists(&self, services: &Vec<ServiceID>) -> Result<bool, Error> {
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

    pub async fn check_if_service_id_online(&self, id: &ServiceID) -> Result<bool, Error> {
        // https://surrealdb.com/docs/surrealql/statements/select
        let q = format!("SELECT VALUE online FROM {}:{};", SERVICE_TABLE, id);

        let mut res = self
            .db
            .query(q)
            .await
            .expect("Failed to check if service id exists");

        let online = res.take(0).expect("Failed to get online status");

        match online {
            None => Ok(false),
            Some(res) => Ok(res),
        }
    }

    pub async fn check_if_services_online(&self, services: &Vec<ServiceID>) -> Result<bool, Error> {
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

    pub async fn read_all_services(&self) -> Result<Vec<ServiceConfig>, Error> {
        let res = self
            .db
            .select(SERVICE_TABLE)
            .await
            .expect("Failed to read all services");

        Ok(res)
    }

    pub async fn read_record_by_id(&self, id: &ServiceID) -> Result<Option<ServiceConfig>, Error> {
        let res = self
            .db
            .select((SERVICE_TABLE, &id.to_string()))
            .await
            .expect("Failed to read service by id");

        Ok(res)
    }

    pub async fn set_service_online(&self, id: &ServiceID) -> Result<bool, Error> {
        self.set_svc_online(id, true).await
    }

    pub async fn set_service_offline(&self, id: &ServiceID) -> Result<bool, Error> {
        self.set_svc_online(id, false).await
    }

    async fn set_svc_online(&self, id: &ServiceID, online: bool) -> Result<bool, Error> {
        // Test if service even exists
        let exists = self
            .check_if_service_id_exists(id)
            .await
            .expect("Failed to check if service id exists");

        match exists {
            false => Ok(false),
            true => {
                //  Update a record with a specific ID
                // let person: Option<Person> = db.update(("person", "tobie"))
                //     .patch(PatchOp::replace("/settings/active", false))
                //     .await?;
                let res: Option<ServiceConfig> = self
                    .db
                    .update((SERVICE_TABLE, &id.to_string()))
                    .patch(PatchOp::replace("online", online))
                    .await
                    .expect("Failed to update and set service online");

                match res {
                    None => Ok(false),
                    Some(_) => Ok(true),
                }
            }
        }
    }

    pub async fn update_service(
        &self,
        data: ServiceConfig,
    ) -> Result<Option<ServiceConfig>, Error> {
        let id = data.svc_id().to_string();

        let updated = self
            .db
            .update((SERVICE_TABLE, id))
            .content(data)
            .await
            .expect("Failed to update service");

        Ok(updated)
    }

    pub async fn delete_service(&self, id: &ServiceID) -> Result<bool, Error> {
        let deleted: Option<ServiceConfig> = self
            .db
            .delete((SERVICE_TABLE, &id.to_string()))
            .await
            .expect("Failed to delete service");

        match deleted {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
