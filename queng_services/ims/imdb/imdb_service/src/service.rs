use proto_imdb::proto::db_gateway_imdb_service_client::DbGatewayImdbServiceClient;
use proto_imdb::proto::imdb_service_server::ImdbService;
use proto_imdb::proto::{
    CheckIfIntegrationConfigExistsRequest, CheckIfIntegrationConfigExistsResponse,
    CheckIfIntegrationConfigOnlineRequest, CheckIfIntegrationConfigOnlineResponse,
    CountIntegrationRequest, CountIntegrationResponse, GetAllIntegrationsByExchangeRequest,
    GetAllIntegrationsByExchangeResponse, GetAllIntegrationsRequest, GetAllIntegrationsResponse,
    GetAllOfflineIntegrationsRequest, GetAllOfflineIntegrationsResponse,
    GetAllOnlineIntegrationsRequest, GetAllOnlineIntegrationsResponse, GetIntegrationConfigRequest,
    GetIntegrationConfigResponse, SetIntegrationOfflineRequest, SetIntegrationOfflineResponse,
    SetIntegrationOnlineRequest, SetIntegrationOnlineResponse,
};
use proto_imdb_utils::{
    get_all_integrations_by_exchange_response, get_all_integrations_response,
    get_all_offline_integrations_response, get_all_online_integrations_response,
    get_check_if_integration_config_exists_response,
    get_check_if_integration_config_online_response, get_count_integration_response,
    get_integration_config_response, get_set_integration_offline_response,
    get_set_integration_online_response,
};
use tonic::transport::Channel;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct IMDBServer {
    dbgw: DbGatewayImdbServiceClient<Channel>,
}

impl IMDBServer {
    pub const fn new(dbgw: DbGatewayImdbServiceClient<Channel>) -> Self {
        Self { dbgw }
    }
}

#[tonic::async_trait]
impl ImdbService for IMDBServer {
    async fn count_integration_configs(
        &self,
        request: Request<CountIntegrationRequest>,
    ) -> Result<Response<CountIntegrationResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.count_integration_configs(request).await {
            Ok(res) => Ok(Response::new(get_count_integration_response(
                res.into_inner().count,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_if_integration_config_exists(
        &self,
        request: Request<CheckIfIntegrationConfigExistsRequest>,
    ) -> Result<Response<CheckIfIntegrationConfigExistsResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.check_if_integration_config_exists(request).await {
            Ok(res) => Ok(Response::new(
                get_check_if_integration_config_exists_response(res.into_inner().exists),
            )),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_if_integration_config_online(
        &self,
        request: Request<CheckIfIntegrationConfigOnlineRequest>,
    ) -> Result<Response<CheckIfIntegrationConfigOnlineResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.check_if_integration_config_online(request).await {
            Ok(res) => Ok(Response::new(
                get_check_if_integration_config_online_response(res.into_inner().online),
            )),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_integration_config(
        &self,
        request: Request<GetIntegrationConfigRequest>,
    ) -> Result<Response<GetIntegrationConfigResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_integration_config(request).await {
            Ok(res) => Ok(Response::new(get_integration_config_response(
                res.into_inner().integration,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_integration_configs(
        &self,
        request: Request<GetAllIntegrationsRequest>,
    ) -> Result<Response<GetAllIntegrationsResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_all_integration_configs(request).await {
            Ok(res) => Ok(Response::new(get_all_integrations_response(
                res.into_inner().integrations,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_integration_configs_by_exchange(
        &self,
        request: Request<GetAllIntegrationsByExchangeRequest>,
    ) -> Result<Response<GetAllIntegrationsByExchangeResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client
            .get_all_integration_configs_by_exchange(request)
            .await
        {
            Ok(res) => Ok(Response::new(get_all_integrations_by_exchange_response(
                res.into_inner().integrations,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_online_integration_configs(
        &self,
        request: Request<GetAllOnlineIntegrationsRequest>,
    ) -> Result<Response<GetAllOnlineIntegrationsResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_all_online_integration_configs(request).await {
            Ok(res) => Ok(Response::new(get_all_online_integrations_response(
                res.into_inner().integrations,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_offline_integration_configs(
        &self,
        request: Request<GetAllOfflineIntegrationsRequest>,
    ) -> Result<Response<GetAllOfflineIntegrationsResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.get_all_offline_integration_configs(request).await {
            Ok(res) => Ok(Response::new(get_all_offline_integrations_response(
                res.into_inner().integrations,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn set_integration_online(
        &self,
        request: Request<SetIntegrationOnlineRequest>,
    ) -> Result<Response<SetIntegrationOnlineResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.set_integration_online(request).await {
            Ok(res) => {
                let inner = res.into_inner();
                Ok(Response::new(get_set_integration_online_response(
                    inner.ok,
                    inner.error,
                )))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn set_integration_offline(
        &self,
        request: Request<SetIntegrationOfflineRequest>,
    ) -> Result<Response<SetIntegrationOfflineResponse>, Status> {
        let mut client = self.dbgw.clone();

        match client.set_integration_offline(request).await {
            Ok(res) => {
                let inner = res.into_inner();
                Ok(Response::new(get_set_integration_offline_response(
                    inner.ok,
                    inner.error,
                )))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
