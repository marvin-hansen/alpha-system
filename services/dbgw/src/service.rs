use autometrics::autometrics;
use common::prelude::ServiceID as RustServiceID;
use common::prelude::{ServiceConfig as RustServiceConfig, ServiceID};
use components::prelude::DBManager;
use dbgw_proto::bindings::db_gateway_service_server::DbGatewayService;
use dbgw_proto::bindings::*;
use surrealdb::Error;
use tonic::{Request, Response, Status};

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
    async fn create_service(
        &self,
        rqt: Request<ProtoServiceConfig>,
    ) -> Result<Response<CreateServiceResponse>, Status> {
        // convert data from proto to Rust via from_proto()
        let data = RustServiceConfig::from_proto(rqt.into_inner())
            .expect("Failed to create ServiceConfig from proto");

        // Write data into DB
        let res = self.dbm.create_service(data).await;

        match res {
            Ok(service_created) => Ok(Response::new(CreateServiceResponse { service_created })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_service_id_exists(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<CheckServiceIdExistsResponse>, Status> {
        // Convert raw integer into ServiceID Enum
        let id = RustServiceID::from(request.into_inner().service_id);

        // Check if the service ID exists in the database
        let res: Result<bool, Error> = self.dbm.check_if_service_id_exists(&id).await;

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
        let proto_services = request.into_inner().services_id;

        let services: Vec<ServiceID> = proto_services.into_iter().map(|x| x.into()).collect();

        let res: Result<bool, Error> = self.dbm.check_if_services_exists(&services).await;

        match res {
            Ok(services_exist) => Ok(Response::new(CheckServicesExistsResponse {
                services_exist,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_service_id_online(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<CheckServiceIdOnlineResponse>, Status> {
        let id = RustServiceID::from(request.into_inner().service_id);

        let res: Result<bool, Error> = self.dbm.check_if_service_id_online(&id).await;

        match res {
            Ok(service_online) => Ok(Response::new(CheckServiceIdOnlineResponse {
                service_online,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_services_online(
        &self,
        request: Request<MultiServicesRequest>,
    ) -> Result<Response<CheckServicesOnlineResponse>, Status> {
        let proto_services = request.into_inner().services_id;

        let services: Vec<ServiceID> = proto_services.into_iter().map(|x| x.into()).collect();

        let res: Result<bool, Error> = self.dbm.check_if_services_online(&services).await;

        match res {
            Ok(services_online) => Ok(Response::new(CheckServicesOnlineResponse {
                services_online,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn read_service(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<ReadServiceResponse>, Status> {
        let id = RustServiceID::from(request.into_inner().service_id);

        let record: Result<Option<RustServiceConfig>, Error> =
            self.dbm.read_record_by_id(&id).await;

        match record {
            Ok(res) => match res {
                None => Ok(Response::new(ReadServiceResponse {
                    service_config: None,
                })),

                Some(res) => {
                    let proto_service_config = res
                        .to_proto()
                        .expect("Failed to convert Rust ServiceConfig to proto");

                    Ok(Response::new(ReadServiceResponse {
                        service_config: Some(proto_service_config),
                    }))
                }
            },
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn read_all_services(
        &self,
        _request: Request<MultiServicesRequest>,
    ) -> Result<Response<ReadAllServicesResponse>, Status> {
        let records: Result<Vec<RustServiceConfig>, Error> = self.dbm.read_all_services().await;

        match records {
            Ok(res) => {
                let mut proto_services = Vec::new();

                if res.is_empty() {
                    Ok(Response::new(ReadAllServicesResponse {
                        service_configs: proto_services,
                    }))
                } else {
                    for record in res {
                        let proto_service_config = record
                            .to_proto()
                            .expect("Failed to convert Rust ServiceConfig to proto");

                        proto_services.push(proto_service_config);
                    }

                    Ok(Response::new(ReadAllServicesResponse {
                        service_configs: proto_services,
                    }))
                }
            }

            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn set_service_online(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<SetServiceOnlineResponse>, Status> {
        let id = RustServiceID::from(request.into_inner().service_id);

        let res: Result<bool, Error> = self.dbm.set_service_online(&id).await;

        match res {
            Ok(service_online) => Ok(Response::new(SetServiceOnlineResponse { service_online })),

            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn set_service_offline(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<SetServiceOfflineResponse>, Status> {
        let id = RustServiceID::from(request.into_inner().service_id);

        let res: Result<bool, Error> = self.dbm.set_service_offline(&id).await;

        match res {
            Ok(service_offline) => Ok(Response::new(SetServiceOfflineResponse { service_offline })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn update_service(
        &self,
        request: Request<ProtoServiceConfig>,
    ) -> Result<Response<UpdateServiceResponse>, Status> {
        let data = RustServiceConfig::from_proto(request.into_inner())
            .expect("Failed to create ServiceConfig from proto");

        let res: Result<Option<RustServiceConfig>, Error> = self.dbm.update_service(data).await;

        match res {
            Ok(res) => match res {
                None => Ok(Response::new(UpdateServiceResponse {
                    service_updated: false,
                })),

                Some(_) => Ok(Response::new(UpdateServiceResponse {
                    service_updated: true,
                })),
            },
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn delete_service(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<DeleteServiceResponse>, Status> {
        let id = RustServiceID::from(request.into_inner().service_id);

        let res: Result<bool, Error> = self.dbm.delete_service(&id).await;

        match res {
            Ok(service_deleted) => Ok(Response::new(DeleteServiceResponse { service_deleted })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
