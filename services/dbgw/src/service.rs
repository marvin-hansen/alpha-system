use std::error::Error as StdError;
use std::fmt;

use serde::{Deserialize, Serialize};
use surrealdb::Error;
use tarpc::context::Context;

use common::prelude::{PortfolioConfig, ServiceConfig, ServiceID};
use components::prelude::DBManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct DBGatewayError(pub String);

impl StdError for DBGatewayError {}

impl fmt::Display for DBGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBGatewayError: {}", self.0)
    }
}


/// Service definition. Client and server are generated from this trait.
#[tarpc::service]
pub trait DBGateway {
    async fn create_portfolio_config(config: PortfolioConfig) -> Result<bool, DBGatewayError>;
    async fn read_all_portfolio_configs() -> Result<Vec<PortfolioConfig>, DBGatewayError>;
    async fn read_portfolio_config_by_id(id: u16) -> Result<Option<PortfolioConfig>, DBGatewayError>;
    async fn update_portfolio_config(data: PortfolioConfig) -> Result<Option<PortfolioConfig>, DBGatewayError>;
    async fn delete_portfolio_config(id: u16) -> Result<bool, DBGatewayError>;
    async fn create_service(data: ServiceConfig) -> Result<bool, DBGatewayError>;
    async fn check_if_service_id_exists(id: ServiceID) -> Result<bool, DBGatewayError>;
    async fn check_if_services_exists(services: Vec<ServiceID>) -> Result<bool, DBGatewayError>;
    async fn check_if_service_id_online(id: ServiceID) -> Result<bool, DBGatewayError>;
    async fn check_if_services_online(id: Vec<ServiceID>) -> Result<bool, DBGatewayError>;
    async fn read_all_services() -> Result<Vec<ServiceConfig>, DBGatewayError>;
    async fn read_service_by_id(id: ServiceID) -> Result<ServiceConfig, DBGatewayError>;
    async fn set_service_online(id: ServiceID) -> Result<bool, DBGatewayError>;
    async fn set_service_offline(id: ServiceID) -> Result<bool, DBGatewayError>;
    async fn update_service(data: ServiceConfig) -> Result<bool, DBGatewayError>;
    async fn delete_service(id: ServiceID) -> Result<bool, DBGatewayError>;
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
    async fn create_portfolio_config(self, _: Context, data: PortfolioConfig) -> Result<bool, DBGatewayError> {
        let created = self.dbm.add_portfolio_config(&data).await;
        match created {
            Ok(res) => Ok(res),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn read_all_portfolio_configs(self, _: Context) -> Result<Vec<PortfolioConfig>, DBGatewayError> {
        let records = self.dbm.read_all_portfolio_configs().await;
        match records {
            Ok(res) => Ok(res),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn read_portfolio_config_by_id(self, _: Context, id: u16) -> Result<Option<PortfolioConfig>, DBGatewayError> {
        let record = self.dbm.read_portfolio_config_by_id(id).await;
        match record {
            Ok(res) => Ok(res),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn update_portfolio_config(self, _: Context, data: PortfolioConfig) -> Result<Option<PortfolioConfig>, DBGatewayError> {
        let id = data.portfolio_id().to_string();
        let updated = self.dbm.update_portfolio_config(data).await;
        match updated {
            Ok(res) => match res {
                None => {
                    Err(DBGatewayError(format!(
                        "Failed to update service id {}",
                        id
                    )))
                }
                Some(res) => Ok(Some(res)),
            },
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn delete_portfolio_config(self, _: Context, id: u16) -> Result<bool, DBGatewayError> {
        let deleted = self.dbm.delete_portfolio_config(id).await;
        match deleted {
            Ok(res) => Ok(res),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }


    async fn create_service(self, _: Context, data: ServiceConfig) -> Result<bool, DBGatewayError> {
        let created: Result<bool, Error> = self.dbm.create_service(data).await;
        match created {
            Ok(res) => Ok(res),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn check_if_service_id_exists(
        self,
        _: Context,
        id: ServiceID,
    ) -> Result<bool, DBGatewayError> {
        let exists: Result<bool, Error> = self.dbm.check_if_service_id_exists(&id).await;
        match exists {
            Ok(res) => Ok(res),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn check_if_services_exists(
        self,
        _: Context,
        services: Vec<ServiceID>,
    ) -> Result<bool, DBGatewayError> {
        let exists: Result<bool, Error> = self.dbm.check_if_services_exists(&services).await;
        match exists {
            Ok(res) => Ok(res),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn check_if_service_id_online(
        self,
        _: Context,
        id: ServiceID,
    ) -> Result<bool, DBGatewayError> {
        let online: Result<bool, Error> = self.dbm.check_if_service_id_online(&id).await;
        match online {
            Ok(res) => Ok(res),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn check_if_services_online(
        self,
        _: Context,
        services: Vec<ServiceID>,
    ) -> Result<bool, DBGatewayError> {
        let online: Result<bool, Error> = self.dbm.check_if_services_online(&services).await;
        match online {
            Ok(res) => Ok(res),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn read_all_services(self, _: Context) -> Result<Vec<ServiceConfig>, DBGatewayError> {
        let records: Result<Vec<ServiceConfig>, Error> = self.dbm.read_all_services().await;
        match records {
            Ok(res) => Ok(res),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn read_service_by_id(
        self,
        _: Context,
        id: ServiceID,
    ) -> Result<ServiceConfig, DBGatewayError> {
        let record: Result<Option<ServiceConfig>, Error> = self.dbm.read_record_by_id(&id).await;
        match record {
            Ok(res) => match res {
                None => Err(DBGatewayError(format!("No record found for id {:?}", id))),
                Some(res) => Ok(res),
            },
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn set_service_online(self, _: Context, id: ServiceID) -> Result<bool, DBGatewayError> {
        let online: Result<bool, Error> = self.dbm.set_service_online(&id).await;
        match online {
            Ok(res) => Ok(res),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn set_service_offline(self, _: Context, id: ServiceID) -> Result<bool, DBGatewayError> {
        let online: Result<bool, Error> = self.dbm.set_service_offline(&id).await;
        match online {
            Ok(res) => Ok(res),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn update_service(self, _: Context, data: ServiceConfig) -> Result<bool, DBGatewayError> {
        let id = data.svc_id().to_string().clone();
        let updated: Result<Option<ServiceConfig>, Error> = self.dbm.update_service(data).await;
        match updated {
            Ok(res) => match res {
                None => Err(DBGatewayError(format!(
                    "Failed to update service id {}",
                    id
                ))),
                Some(_) => Ok(true),
            },
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }

    async fn delete_service(self, _: Context, id: ServiceID) -> Result<bool, DBGatewayError> {
        let deleted: Result<bool, Error> = self.dbm.delete_service(&id).await;
        match deleted {
            Ok(res) => Ok(res),
            Err(e) => Err(DBGatewayError(e.to_string())),
        }
    }
}
