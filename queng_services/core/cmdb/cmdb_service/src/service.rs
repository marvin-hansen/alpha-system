use pg_cmdb_manager::PostgresCMDBManager;
use proto_bindings::proto::cmdb_service_server::CmdbService;
use proto_bindings::proto::*;
use proto_utils::portfolio_proto_utils::{portfolio_config_from_proto, portfolio_config_to_proto};
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

pub(crate) type SafePostgresDBManager = Arc<RwLock<PostgresCMDBManager>>;

#[derive(Clone)]
pub struct CMDBServer {
    dbm: SafePostgresDBManager,
}

impl CMDBServer {
    pub fn new(dbm: SafePostgresDBManager) -> Self {
        Self { dbm }
    }
}

#[tonic::async_trait]
impl CmdbService for CMDBServer {
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
}
