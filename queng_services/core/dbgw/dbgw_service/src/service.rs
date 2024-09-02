use std::sync::Arc;

use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

use common_config::prelude::ServiceID;
use pg_smdb_manager::PostgresSMDBManager;
use proto_smdb::proto::db_gateway_service_server::DbGatewayService;
use proto_smdb::proto::*;
use proto_utils::endpoint_proto_utils::endpoint_to_proto;
use proto_utils::service_config_proto_utils::{
    service_config_collection_to_proto, service_config_from_proto, service_config_to_proto,
};

use crate::DBG;

pub(crate) type SafePostgresDBManager = Arc<RwLock<PostgresSMDBManager>>;

#[derive(Clone)]
pub struct DBGWServer {
    dbg: bool,
    dbm: SafePostgresDBManager,
}

impl DBGWServer {
    pub fn new(dbm: SafePostgresDBManager) -> Self {
        Self { dbg: DBG, dbm }
    }
}

#[tonic::async_trait]
impl DbGatewayService for DBGWServer {
    async fn create_service(
        &self,
        rqt: Request<ProtoServiceConfig>,
    ) -> Result<Response<CreateServiceResponse>, Status> {
        self.dbg_print("create_service");

        let data = service_config_from_proto(rqt.into_inner())
            .expect("Failed to create ServiceConfig from proto");

        let dbm = self.dbm.write().await;
        let res = dbm.insert_service(&data).await;

        match res {
            Ok(_) => Ok(Response::new(CreateServiceResponse {
                service_created: true,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn count_all_services(
        &self,
        _request: Request<CountServiceRequest>,
    ) -> Result<Response<CountServiceResponse>, Status> {
        self.dbg_print("count_all_services");

        let mut dbm = self.dbm.write().await;
        let res = dbm.count_services().await;

        match res {
            Ok(service_count) => Ok(Response::new(CountServiceResponse { service_count })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_service_id_exists(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<CheckServiceIdExistsResponse>, Status> {
        self.dbg_print("check_service_id_exists");

        let id = ServiceID::from(request.into_inner().service_id);

        let dbm = self.dbm.read().await;
        let res = dbm.check_if_service_id_exists(&id).await;

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
        self.dbg_print("check_services_exists");

        let proto_services = request.into_inner().services_id;

        let services: Vec<ServiceID> = proto_services.into_iter().map(|x| x.into()).collect();
        let dbm = self.dbm.read().await;

        let res = dbm.check_if_services_exists(&services).await;

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
        self.dbg_print("check_service_id_online");

        let id = ServiceID::from(request.into_inner().service_id);

        let dbm = self.dbm.read().await;
        let res = dbm.check_if_service_id_online(&id).await;

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
        self.dbg_print("check_services_online");

        let proto_services = request.into_inner().services_id;

        let services = proto_services.into_iter().map(|x| x.into()).collect();

        let dbm = self.dbm.read().await;
        let res = dbm.check_if_services_online(&services).await;

        match res {
            Ok(services_online) => Ok(Response::new(CheckServicesOnlineResponse {
                services_online,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_service_dependencies(
        &self,
        request: Request<ServiceDependenciesRequest>,
    ) -> Result<Response<ServiceDependenciesResponse>, Status> {
        let dbm = self.dbm.read().await;

        let id = ServiceID::from(request.into_inner().service_id);

        let records = dbm.get_all_service_dependencies(&id).await;
        match records {
            Ok(res) => {
                if res.is_empty() {
                    Ok(Response::new(ServiceDependenciesResponse {
                        service_id: id.as_i32(),
                        dependencies: vec![],
                    }))
                } else {
                    // Convert Service IDs to i32
                    let dependencies: Vec<i32> = res.iter().map(|x| x.as_i32()).collect();

                    Ok(Response::new(ServiceDependenciesResponse {
                        service_id: id.as_i32(),
                        dependencies,
                    }))
                }
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_service_endpoints(
        &self,
        request: Request<ServiceEndpointsRequest>,
    ) -> Result<Response<ServiceEndpointsResponse>, Status> {
        let dbm = self.dbm.read().await;

        let id = ServiceID::from(request.into_inner().service_id);

        let records = dbm.get_all_service_endpoints(&id).await;

        match records {
            Ok(res) => {
                if res.is_empty() {
                    Ok(Response::new(ServiceEndpointsResponse {
                        service_id: id.as_i32(),
                        endpoints: vec![],
                    }))
                } else {
                    // Convert endpoints to proto
                    let endpoints = endpoint_to_proto(&res)
                        .expect("Failed to convert Rust EndpointCollection to proto");

                    Ok(Response::new(ServiceEndpointsResponse {
                        service_id: id.as_i32(),
                        endpoints,
                    }))
                }
            }

            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_online_services(
        &self,
        _request: Request<ServicesOnlineRequest>,
    ) -> Result<Response<ServicesOnlineResponse>, Status> {
        let dbm = self.dbm.read().await;
        let records = dbm.get_all_online_services().await;

        match records {
            Ok(res) => {
                if res.is_empty() {
                    Ok(Response::new(ServicesOnlineResponse {
                        service_configs: Vec::new(),
                    }))
                } else {
                    let proto_service_configs = service_config_collection_to_proto(&res)
                        .expect("Failed to convert Rust ServiceConfigCollection to proto");

                    Ok(Response::new(ServicesOnlineResponse {
                        service_configs: proto_service_configs,
                    }))
                }
            }

            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_offline_services(
        &self,
        _request: Request<ServicesOfflineRequest>,
    ) -> Result<Response<ServicesOfflineResponse>, Status> {
        let dbm = self.dbm.read().await;
        let records = dbm.get_all_offline_services().await;

        match records {
            Ok(res) => {
                if res.is_empty() {
                    Ok(Response::new(ServicesOfflineResponse {
                        service_configs: Vec::new(),
                    }))
                } else {
                    let proto_service_configs = service_config_collection_to_proto(&res)
                        .expect("Failed to convert Rust ServiceConfigCollection to proto");

                    Ok(Response::new(ServicesOfflineResponse {
                        service_configs: proto_service_configs,
                    }))
                }
            }

            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn read_service(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<ReadServiceResponse>, Status> {
        self.dbg_print("read_service");

        let id = ServiceID::from(request.into_inner().service_id);

        let dbm = self.dbm.read().await;
        let record = dbm.read_service_by_id(&id).await;

        match record {
            Ok(res) => match res {
                None => Ok(Response::new(ReadServiceResponse {
                    service_config: None,
                })),

                Some(res) => {
                    let proto_service_config = service_config_to_proto(res)
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
        self.dbg_print("read_all_services");

        let dbm = self.dbm.read().await;
        let records = dbm.read_all_services().await;

        match records {
            Ok(res) => {
                if res.is_empty() {
                    Ok(Response::new(ReadAllServicesResponse {
                        service_configs: Vec::new(),
                    }))
                } else {
                    let proto_service_configs = service_config_collection_to_proto(&res)
                        .expect("Failed to convert Rust ServiceConfigCollection to proto");

                    Ok(Response::new(ReadAllServicesResponse {
                        service_configs: proto_service_configs,
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
        self.dbg_print("set_service_online");

        let id = ServiceID::from(request.into_inner().service_id);

        let dbm = self.dbm.write().await;
        let res = dbm.set_service_online(&id).await;

        match res {
            Ok(_) => Ok(Response::new(SetServiceOnlineResponse {
                service_online: true,
            })),

            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn set_service_offline(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<SetServiceOfflineResponse>, Status> {
        self.dbg_print("set_service_offline");

        let id = ServiceID::from(request.into_inner().service_id);

        let dbm = self.dbm.write().await;
        let res = dbm.set_service_offline(&id).await;

        match res {
            Ok(_) => Ok(Response::new(SetServiceOfflineResponse {
                service_offline: true,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn update_service(
        &self,
        request: Request<ProtoServiceConfig>,
    ) -> Result<Response<UpdateServiceResponse>, Status> {
        self.dbg_print("update_service");

        let data = service_config_from_proto(request.into_inner())
            .expect("Failed to create ServiceConfig from proto");

        let dbm = self.dbm.write().await;
        let res = dbm.update_service(data).await;

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
        self.dbg_print("delete_service");

        let id = ServiceID::from(request.into_inner().service_id);

        let dbm = self.dbm.write().await;
        let res = dbm.delete_service(&id).await;

        match res {
            Ok(service_deleted) => Ok(Response::new(DeleteServiceResponse { service_deleted })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}

impl DBGWServer {
    fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[DBGW/service]: {}", msg)
        }
    }
}
