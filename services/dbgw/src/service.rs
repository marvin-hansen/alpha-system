use surrealdb::Error;
use autometrics::autometrics;
use tonic::{Request, Response, Status};
use components::prelude::DBManager;
use common::prelude::ServiceID as RustServiceID;
use common::prelude::ServiceConfig as RustServiceConfig;
use dbgw::db_gateway_service_server::{DbGatewayService};
use dbgw::*;

pub mod dbgw {
    tonic::include_proto!("dbgw");
}

#[derive(Debug, Default)]
pub struct DBGWServer {
    dbm: DBManager,
}

impl DBGWServer {
    pub fn new(dbm: DBManager) -> Self {
        Self { dbm }
    }
}

#[tonic::async_trait]
#[autometrics]
impl DbGatewayService for DBGWServer {
    async fn create_service(&self, _request: Request<ServiceConfig>) -> Result<Response<CreateServiceResponse>, Status> {

        // let data = request.into_inner();

        // convert data from proto to Rust
        let s = RustServiceConfig::default();

        let created = self.dbm.create_service(s).await;

        return if created.is_ok() {
            Ok(Response::new(CreateServiceResponse { success: true }))
        } else {
            Ok(Response::new(CreateServiceResponse { success: false }))
        };
    }

    async fn check_service_id_exists(&self, request: Request<SingleServiceRequest>) -> Result<Response<CheckServiceIdExistsResponse>, Status> {

        let req = request.into_inner().service_id;

        let id = RustServiceID::from(req);

        let exists: Result<bool, Error> = self.dbm.check_if_service_id_exists(&id).await;

        return if exists.is_ok() {
            Ok(Response::new(CheckServiceIdExistsResponse { service_exists: true }))
        } else {
            // Add proper error handling here
            Ok(Response::new(CheckServiceIdExistsResponse { service_exists: false }))
        };
    }

    async fn check_services_exists(&self, _request: Request<MultiServicesRequest>) -> Result<Response<CheckServicesExistsResponse>, Status> {
        todo!()
    }

    async fn check_service_id_online(&self, _request: Request<SingleServiceRequest>) -> Result<Response<CheckServiceIdOnlineResponse>, Status> {
        todo!()
    }

    async fn check_services_online(&self, _request: Request<MultiServicesRequest>) -> Result<Response<CheckServicesOnlineResponse>, Status> {
        todo!()
    }

    async fn read_service(&self, _request: Request<SingleServiceRequest>) -> Result<Response<ReadServiceResponse>, Status> {
        todo!()
    }

    async fn read_all_services(&self, _request: Request<MultiServicesRequest>) -> Result<Response<ReadAllServicesResponse>, Status> {
        todo!()
    }

    async fn set_service_online(&self, _request: Request<SingleServiceRequest>) -> Result<Response<SetServiceOnlineResponse>, Status> {
        todo!()
    }

    async fn set_service_offline(&self, _request: Request<MultiServicesRequest>) -> Result<Response<SetServiceOfflineResponse>, Status> {
        todo!()
    }

    async fn update_service(&self, _request: Request<ServiceConfig>) -> Result<Response<UpdateServiceResponse>, Status> {
        todo!()
    }

    async fn delete_service(&self, _request: Request<SingleServiceRequest>) -> Result<Response<DeleteServiceResponse>, Status> {
        todo!()
    }
}

