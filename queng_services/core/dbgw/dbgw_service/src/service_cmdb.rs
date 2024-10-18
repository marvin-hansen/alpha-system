use pg_cmdb_manager::PostgresCMDBManager;
use proto_cmdb::proto::db_gateway_cmdb_service_server::DbGatewayCmdbService;

use crate::DBG;
use proto_cmdb::proto::{
    CreatePortfolioResponse, DeletePortfolioResponse, MultiPortfolioRequest, ProtoPortfolioConfig,
    ReadAllPortfoliosResponse, ReadPortfolioResponse, SinglePortfolioRequest,
    UpdatePortfolioResponse,
};
use proto_cmdb_utils::portfolio_proto_utils::{
    portfolio_config_from_proto, portfolio_config_to_proto,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

pub(crate) type SafePgCMDBManager = Arc<RwLock<PostgresCMDBManager>>;

#[derive(Clone)]
pub struct CMDBServer {
    dbg: bool,
    dbm: SafePgCMDBManager,
}

impl CMDBServer {
    pub fn new(dbm: SafePgCMDBManager) -> Self {
        Self { dbg: DBG, dbm }
    }

    fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[DBGW/service_cmdb]: {}", msg)
        }
    }
}

#[tonic::async_trait]
impl DbGatewayCmdbService for CMDBServer {
    async fn create_portfolio_config(
        &self,
        request: Request<ProtoPortfolioConfig>,
    ) -> Result<Response<CreatePortfolioResponse>, Status> {
        self.dbg_print("create_portfolio_config");

        let data =
            portfolio_config_from_proto(request.into_inner()).expect("Failed to parse request");

        let dbm = self.dbm.write().await;
        match dbm.insert_portfolio_config(&data).await {
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
        self.dbg_print("read_portfolio_config");

        let id = request.into_inner().portfolio_id as u16;

        let dbm = self.dbm.read().await;
        match dbm.read_portfolio_config_by_id(id).await {
            Ok(res) => {
                let proto_portfolio_config =
                    portfolio_config_to_proto(res).expect("Failed to convert record to proto");

                Ok(Response::new(ReadPortfolioResponse {
                    portfolio_config: Some(proto_portfolio_config),
                }))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn read_all_portfolio_configs(
        &self,
        _request: Request<MultiPortfolioRequest>,
    ) -> Result<Response<ReadAllPortfoliosResponse>, Status> {
        self.dbg_print("read_all_portfolio_configs");

        let dbm = self.dbm.read().await;

        match dbm.read_all_portfolio_configs().await {
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
        self.dbg_print("update_portfolio_config");

        let data =
            portfolio_config_from_proto(request.into_inner()).expect("Failed to parse request");

        let dbm = self.dbm.write().await;
        match dbm.update_portfolio_config(data).await {
            Ok(_) => Ok(Response::new(UpdatePortfolioResponse {
                portfolio_updated: true,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn delete_portfolio_config(
        &self,
        request: Request<SinglePortfolioRequest>,
    ) -> Result<Response<DeletePortfolioResponse>, Status> {
        self.dbg_print("delete_portfolio_config");

        let id = request.into_inner().portfolio_id as u16;

        let dbm = self.dbm.write().await;
        match dbm.delete_portfolio_config(id).await {
            Ok(portfolio_deleted) => {
                Ok(Response::new(DeletePortfolioResponse { portfolio_deleted }))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
