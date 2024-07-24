use std::sync::Arc;

use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

use common_config::prelude::ServiceID;
use db_postgres_manager::prelude::PostgresDBManager;
use proto_bindings::proto::db_gateway_service_server::DbGatewayService;
use proto_bindings::proto::*;
use proto_utils::portfolio_proto_utils::{portfolio_config_from_proto, portfolio_config_to_proto};
use proto_utils::service_config_proto_utils::{service_config_from_proto, service_config_to_proto};

pub(crate) type SafePostgresDBManager = Arc<RwLock<PostgresDBManager>>;

pub struct DBGWServer {
    dbm: SafePostgresDBManager,
}

impl DBGWServer {
    pub fn new(dbm: SafePostgresDBManager) -> Self {
        Self { dbm }
    }
}

#[tonic::async_trait]
impl DbGatewayService for DBGWServer {
    async fn create_portfolio_config(
        &self,
        request: Request<ProtoPortfolioConfig>,
    ) -> Result<Response<CreatePortfolioResponse>, Status> {
        let data =
            portfolio_config_from_proto(request.into_inner()).expect("Failed to parse request");

        let dbm = self.dbm.write().await;
        let res = dbm.insert_portfolio_config(&data).await;

        match res {
            Ok(_) => Ok(Response::new(CreatePortfolioResponse {
                portfolio_created: true,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn read_portfolio_config(
        &self,
        request: Request<SinglePortfolioRequest>,
    ) -> Result<Response<ReadPortfolioResponse>, Status> {
        let id = request.into_inner().portfolio_id as u16;

        let dbm = self.dbm.read().await;
        let record = dbm.read_portfolio_config_by_id(id).await;

        match record {
            Ok(res) => match res {
                None => Ok(Response::new(ReadPortfolioResponse {
                    portfolio_config: None,
                })),
                Some(res) => {
                    let proto_portfolio_config =
                        portfolio_config_to_proto(res).expect("Failed to convert record to proto");

                    Ok(Response::new(ReadPortfolioResponse {
                        portfolio_config: Some(proto_portfolio_config),
                    }))
                }
            },
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn read_all_portfolio_configs(
        &self,
        _request: Request<MultiPortfolioRequest>,
    ) -> Result<Response<ReadAllPortfoliosResponse>, Status> {
        //
        let dbm = self.dbm.read().await;
        let records = dbm.read_all_portfolio_configs().await;

        match records {
            Ok(res) => {
                let mut portfolio_configs: Vec<ProtoPortfolioConfig> = Vec::new();

                if res.is_empty() {
                    Ok(Response::new(ReadAllPortfoliosResponse {
                        portfolio_configs,
                    }))
                } else {
                    for record in res {
                        let proto_portfolio_config = portfolio_config_to_proto(record)
                            .expect("Failed to convert record to proto");

                        portfolio_configs.push(proto_portfolio_config);
                    }

                    Ok(Response::new(ReadAllPortfoliosResponse {
                        portfolio_configs,
                    }))
                }
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn update_portfolio_config(
        &self,
        request: Request<ProtoPortfolioConfig>,
    ) -> Result<Response<UpdatePortfolioResponse>, Status> {
        let data =
            portfolio_config_from_proto(request.into_inner()).expect("Failed to parse request");

        let dbm = self.dbm.write().await;
        let res = dbm.update_portfolio_config(data).await;

        match res {
            Ok(res) => match res {
                None => Ok(Response::new(UpdatePortfolioResponse {
                    portfolio_updated: false,
                })),
                Some(_) => Ok(Response::new(UpdatePortfolioResponse {
                    portfolio_updated: true,
                })),
            },
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn delete_portfolio_config(
        &self,
        request: Request<SinglePortfolioRequest>,
    ) -> Result<Response<DeletePortfolioResponse>, Status> {
        let id = request.into_inner().portfolio_id as u16;

        let dbm = self.dbm.write().await;
        let res = dbm.delete_portfolio_config(id).await;

        match res {
            Ok(portfolio_deleted) => {
                Ok(Response::new(DeletePortfolioResponse { portfolio_deleted }))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn create_service(
        &self,
        rqt: Request<ProtoServiceConfig>,
    ) -> Result<Response<CreateServiceResponse>, Status> {
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

    async fn check_service_id_exists(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<CheckServiceIdExistsResponse>, Status> {
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

    async fn read_service(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<ReadServiceResponse>, Status> {
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
        let dbm = self.dbm.read().await;
        let records = dbm.read_all_services().await;

        match records {
            Ok(res) => {
                let mut service_configs = Vec::new();

                if res.is_empty() {
                    Ok(Response::new(ReadAllServicesResponse { service_configs }))
                } else {
                    for record in res {
                        let proto_service_config = service_config_to_proto(record)
                            .expect("Failed to convert Rust ServiceConfig to proto");

                        service_configs.push(proto_service_config);
                    }

                    Ok(Response::new(ReadAllServicesResponse { service_configs }))
                }
            }

            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn set_service_online(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<SetServiceOnlineResponse>, Status> {
        let id = ServiceID::from(request.into_inner().service_id);

        let dbm = self.dbm.write().await;
        let res = dbm.set_service_online(&id).await;

        match res {
            Ok(service_online) => Ok(Response::new(SetServiceOnlineResponse { service_online })),

            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn set_service_offline(
        &self,
        request: Request<SingleServiceRequest>,
    ) -> Result<Response<SetServiceOfflineResponse>, Status> {
        let id = ServiceID::from(request.into_inner().service_id);

        let dbm = self.dbm.write().await;
        let res = dbm.set_service_offline(&id).await;

        match res {
            Ok(service_offline) => Ok(Response::new(SetServiceOfflineResponse { service_offline })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn update_service(
        &self,
        request: Request<ProtoServiceConfig>,
    ) -> Result<Response<UpdateServiceResponse>, Status> {
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
        let id = ServiceID::from(request.into_inner().service_id);

        let dbm = self.dbm.write().await;
        let res = dbm.delete_service(&id).await;

        match res {
            Ok(service_deleted) => Ok(Response::new(DeleteServiceResponse { service_deleted })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
