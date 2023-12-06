use std::sync::{Arc, Mutex};
use proto::binding::db_gateway_service_client::DbGatewayServiceClient;
// use autometrics::autometrics;
use tonic::{Request, Response, Status};
use common::prelude::ServiceID;
use dbgw_client::DBGatewayClient;
use proto::binding::*;
use proto::binding::smdb_service_server::SmdbService;

#[derive(Clone)]
pub struct SMDBServer {
    // https://stackoverflow.com/questions/67877287/refcellstdstringstring-cannot-be-shared-between-threads-safely
    dbgw: Arc<Mutex<DBGatewayClient>>,
}

impl SMDBServer {
    pub fn new(dbgw:  Arc<Mutex<DBGatewayClient>>) -> Self {
        Self { dbgw }
    }
}

#[tonic::async_trait]
// #[autometrics]
impl SmdbService for SMDBServer {
    async fn check_service_id_exists(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<CheckServiceIdExistsResponse>, Status> {

        let id = ServiceID::from(request.into_inner().service_id);

        let mut binding = self.dbgw.lock().unwrap();

        let fut =  binding.check_if_service_id_exists(id);

        let res = fut.await;

        match res {
            Ok(service_exists) => Ok(Response::new(CheckServiceIdExistsResponse {
                service_exists,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_services_exists(
        &self,
        request: Request<MultiServicesRequest>,
    ) -> Result<Response<CheckServicesExistsResponse>, Status> {

        // let proto_services = request.into_inner().services_id;

        // let services: Vec<ServiceID> = proto_services.into_iter().map(|x| x.into()).collect();
        //
        // let client = self.dbgw.lock().unwrap();
        //
        // let res = client.check_if_services_exists(services).await;
        // match res {
        //     Ok(services_exist) => Ok(Response::new(CheckServicesExistsResponse {
        //         services_exist,
        //     })),
        //     Err(e) => Err(Status::internal(e.to_string())),
        // }

        todo!()
    }

    async fn check_service_id_online(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<CheckServiceIdOnlineResponse>, Status> {
        // let id = ServiceID::from(request.into_inner().service_id);
        //
        // let client = self.dbgw.lock().unwrap();
        //
        // let res = client.check_if_service_id_online(id).await;
        //
        // match res {
        //     Ok(service_online) => Ok(Response::new(CheckServiceIdOnlineResponse {
        //         service_online,
        //     })),
        //     Err(e) => Err(Status::internal(e.to_string())),
        // }

        todo!()
    }

    async fn check_services_online(
        &self,
        request: Request<MultiServicesRequest>,
    ) -> Result<Response<CheckServicesOnlineResponse>, Status> {
        // let proto_services = request.into_inner().services_id;
        // let services: Vec<ServiceID> = proto_services.into_iter().map(|x| x.into()).collect();
        //
        // let client = self.dbgw.lock().unwrap();
        //
        // let res = client.check_if_services_online(services).await;
        //
        // match res {
        //     Ok(services_online) => Ok(Response::new(CheckServicesOnlineResponse {
        //         services_online,
        //     })),
        //     Err(e) => Err(Status::internal(e.to_string())),
        // }

        todo!()
    }

    async fn set_service_online(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<SetServiceOnlineResponse>, Status> {
        // let id = ServiceID::from(request.into_inner().service_id);
        //
        // let client = self.dbgw.lock().unwrap();
        //
        // let res = client.set_service_online(id).await;
        //
        // match res {
        //     Ok(service_online) => Ok(Response::new(SetServiceOnlineResponse { service_online })),
        //     Err(e) => Err(Status::internal(e.to_string())),
        // }

        todo!()
    }

    async fn set_service_offline(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<SetServiceOfflineResponse>, Status> {
        // let id = ServiceID::from(request.into_inner().service_id);
        // let client = self.dbgw.lock().unwrap();
        //
        // let res = client.set_service_offline(id).await;
        //
        // match res {
        //     Ok(service_offline) => Ok(Response::new(SetServiceOfflineResponse { service_offline })),
        //     Err(e) => Err(Status::internal(e.to_string())),
        // }

        todo!()
    }
}
