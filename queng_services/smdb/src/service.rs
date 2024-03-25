use tonic::transport::Channel;
use tonic::{Request, Response, Status};

use proto_bindings::proto::db_gateway_service_client::DbGatewayServiceClient;
use proto_bindings::proto::smdb_service_server::SmdbService;
use proto_bindings::proto::*;

use crate::SVC_ID;

#[derive(Clone)]
pub struct SMDBServer {
    dbgw: DbGatewayServiceClient<Channel>,
}

impl SMDBServer {
    pub fn new(dbgw: DbGatewayServiceClient<Channel>) -> Self {
        Self { dbgw }
    }
}

#[tonic::async_trait]
impl SmdbService for SMDBServer {
    async fn check_service_id_exists(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<CheckServiceIdExistsResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.check_service_id_exists(request).await {
            Ok(res) => Ok(Response::new(CheckServiceIdExistsResponse {
                service_exists: res.into_inner().service_exists,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_services_exists(
        &self,
        request: Request<MultiServicesRequest>,
    ) -> Result<Response<CheckServicesExistsResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.check_services_exists(request).await {
            Ok(res) => Ok(Response::new(CheckServicesExistsResponse {
                services_exist: res.into_inner().services_exist,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_service_id_online(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<CheckServiceIdOnlineResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.check_service_id_online(request).await {
            Ok(res) => Ok(Response::new(CheckServiceIdOnlineResponse {
                service_online: res.into_inner().service_online,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_services_online(
        &self,
        request: Request<MultiServicesRequest>,
    ) -> Result<Response<CheckServicesOnlineResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.check_services_online(request).await {
            Ok(res) => Ok(Response::new(CheckServicesOnlineResponse {
                services_online: res.into_inner().services_online,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn set_service_online(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<SetServiceOnlineResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.set_service_online(request).await {
            Ok(res) => Ok(Response::new(SetServiceOnlineResponse {
                service_online: res.into_inner().service_online,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn set_service_offline(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<SetServiceOfflineResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.set_service_offline(request).await {
            Ok(res) => Ok(Response::new(SetServiceOfflineResponse {
                service_offline: res.into_inner().service_offline,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}

pub fn get_svc_request() -> Request<SingleServiceRequest> {
    Request::new(SingleServiceRequest {
        service_id: SVC_ID as i32,
    })
}
