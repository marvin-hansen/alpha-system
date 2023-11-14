use surrealdb::Error;
use tarpc::context::Context;

use common::prelude::{ServiceConfig, ServiceID};
use components::prelude::DBManager;

/// This is the service definition. It looks a lot like a trait definition.
/// It defines one RPC, hello, which takes one arg, name, and returns a String.
pub const PORT: u16 = 8080;

#[tarpc::service]
pub trait DBGateway {
    async fn create_service(data: ServiceConfig) -> bool;
    async fn read_all_services() -> Option<Vec<ServiceConfig>>;
    async fn read_record_by_id(id: ServiceID) -> Option<ServiceConfig>;
    async fn update_service(data: ServiceConfig) -> Option<ServiceConfig>;
    async fn delete_service(id: ServiceID) -> bool;
}

// This is the type that implements the generated World trait.
// It is the business logic and is used to start the server.
#[derive(Clone)]
pub struct DBGatewayServer {
    dbm: DBManager,
}

impl DBGatewayServer {
    pub fn new(dbm: DBManager) -> Self {
        Self { dbm }
    }
}

#[tarpc::server]
impl DBGateway for DBGatewayServer {
    async fn create_service(self, _: Context, data: ServiceConfig) -> bool {
        let created: Result<bool, Error> = self.dbm.create_service(data).await;
        match created {
            Ok(created) => created,
            Err(e) => {
                println!("Error: {:?}", e);
                false
            }
        }
    }

    async fn read_all_services(self, _: Context) -> Option<Vec<ServiceConfig>> {
        let records = self.dbm.read_all_services().await;
        match records {
            Ok(records) => Some(records),
            Err(_) => None,
        }
    }

    async fn read_record_by_id(self, _: Context, id: ServiceID) -> Option<ServiceConfig> {
        let record: Result<Option<ServiceConfig>, Error> = self.dbm.read_record_by_id(&id).await;
        match record {
            Ok(res) => res,
            Err(_) => None,
        }
    }

    async fn update_service(self, _: Context, data: ServiceConfig) -> Option<ServiceConfig> {
        let updated: Result<Option<ServiceConfig>, Error> = self.dbm.update_service(data).await;
        match updated {
            Ok(res) => res,
            Err(_) => None,
        }
    }

    async fn delete_service(self, _: Context, id: ServiceID) -> bool {
        let deleted: Result<bool, Error> = self.dbm.delete_service(&id).await;
        match deleted {
            Ok(deleted) => deleted,
            Err(e) => {
                println!("Error: {:?}", e);
                false
            }
        }
    }
}
