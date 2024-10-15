use proto_cmdb::proto::cmdb_service_server::CmdbService;
use proto_cmdb::proto::*;

use proto_dbgw::proto::db_gateway_cmdb_service_client::DbGatewayCmdbServiceClient;
use tonic::transport::Channel;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct CMDBServer {
    dbgw: DbGatewayCmdbServiceClient<Channel>,
}

impl CMDBServer {
    pub fn new(dbgw: DbGatewayCmdbServiceClient<Channel>) -> Self {
        Self { dbgw }
    }
}

#[tonic::async_trait]
impl CmdbService for CMDBServer {
    async fn create_portfolio_config(
        &self,
        request: Request<ProtoPortfolioConfig>,
    ) -> Result<Response<CreatePortfolioResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.create_portfolio_config(request).await {
            Ok(res) => Ok(Response::new(CreatePortfolioResponse {
                portfolio_created: res.into_inner().portfolio_created,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn read_portfolio_config(
        &self,
        request: Request<SinglePortfolioRequest>,
    ) -> Result<Response<ReadPortfolioResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.read_portfolio_config(request).await {
            Ok(res) => Ok(Response::new(ReadPortfolioResponse {
                portfolio_config: res.into_inner().portfolio_config,
            })),

            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn read_all_portfolio_configs(
        &self,
        request: Request<MultiPortfolioRequest>,
    ) -> Result<Response<ReadAllPortfoliosResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.read_all_portfolio_configs(request).await {
            Ok(res) => Ok(Response::new(ReadAllPortfoliosResponse {
                portfolio_configs: res.into_inner().portfolio_configs,
            })),

            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn update_portfolio_config(
        &self,
        request: Request<ProtoPortfolioConfig>,
    ) -> Result<Response<UpdatePortfolioResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.update_portfolio_config(request).await {
            Ok(res) => Ok(Response::new(UpdatePortfolioResponse {
                portfolio_updated: res.into_inner().portfolio_updated,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn delete_portfolio_config(
        &self,
        request: Request<SinglePortfolioRequest>,
    ) -> Result<Response<DeletePortfolioResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.delete_portfolio_config(request).await {
            Ok(res) => Ok(Response::new(DeletePortfolioResponse {
                portfolio_deleted: res.into_inner().portfolio_deleted,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
