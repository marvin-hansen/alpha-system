use surrealdb::engine::local;
use surrealdb::Error;
use surrealdb::Surreal;

use common::prelude::{ServiceConfig, ServiceID};

const SERVICE_TABLE: &str = "service";

#[derive(Clone)]
pub struct DBManager {
    db: Surreal<local::Db>,
}

impl DBManager {
    pub fn new(db: Surreal<local::Db>) -> Self {
        // local DB is either in memory or flat file on disk so it's always connected
        Self {
            db,
        }
    }
}

impl DBManager {
    pub async fn create_service(&self, data: ServiceConfig) -> Result<bool, Error> {
        let table = SERVICE_TABLE;
        let id = data.svc_id().to_string();

        let created: Option<ServiceConfig> = self.db.update((table, id)).merge(data).await?;
        match created {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    pub async fn read_all_services(&self) -> Result<Vec<ServiceConfig>, Error> {
        let res = self.db.select(SERVICE_TABLE).await?;
        Ok(res)
    }

    pub async fn read_record_by_id(&self, id: &ServiceID) -> Result<Option<ServiceConfig>, Error> {
        let res: Option<ServiceConfig> = self.db.select((SERVICE_TABLE, &id.to_string())).await?;
        Ok(res)
    }

    pub async fn update_service(
        &self,
        data: ServiceConfig,
    ) -> Result<Option<ServiceConfig>, Error> {
        let id = data.svc_id().to_string();
        let updated: Option<ServiceConfig> =
            self.db.update((SERVICE_TABLE, id)).content(data).await?;
        Ok(updated)
    }

    pub async fn delete_service(&self, id: &ServiceID) -> Result<bool, Error> {
        let deleted: Option<ServiceConfig> =
            self.db.delete((SERVICE_TABLE, &id.to_string())).await?;
        match deleted {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
