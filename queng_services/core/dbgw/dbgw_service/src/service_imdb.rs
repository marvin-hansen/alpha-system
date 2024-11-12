use crate::DBG;
use pg_imdb_manager::PostgresIMDBManager;
use proto_imdb::proto::db_gateway_imdb_service_server::DbGatewayImdbService;
use proto_imdb::proto::*;
use proto_imdb_utils::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

pub(crate) type SafePgIMDBManager = Arc<RwLock<PostgresIMDBManager>>;

#[derive(Clone)]
pub struct IMDBServer {
    dbg: bool,
    dbm: SafePgIMDBManager,
}

impl IMDBServer {
    pub fn new(dbm: SafePgIMDBManager) -> Self {
        Self { dbg: DBG, dbm }
    }

    fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[DBGW/service_imdb]: {}", msg)
        }
    }
}

#[tonic::async_trait]
impl DbGatewayImdbService for IMDBServer {
    async fn create_integration_config(
        &self,
        request: Request<CreateIntegrationRequest>,
    ) -> Result<Response<CreateIntegrationResponse>, Status> {
        self.dbg_print("create_integration_config");

        let data = integration_config_from_proto(request.into_inner().integration.unwrap());

        let dbm = self.dbm.write().await;
        match dbm.insert_integration_config(data).await {
            Ok(_) => Ok(Response::new(get_create_integration_response())),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn count_integration_configs(
        &self,
        _: Request<CountIntegrationRequest>,
    ) -> Result<Response<CountIntegrationResponse>, Status> {
        self.dbg_print("count_integration_configs");

        let dbm = self.dbm.write().await;
        match dbm.count_integration_configs().await {
            Ok(nr) => Ok(Response::new(get_count_integration_response(nr))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_if_integration_config_exists(
        &self,
        request: Request<CheckIfIntegrationConfigExistsRequest>,
    ) -> Result<Response<CheckIfIntegrationConfigExistsResponse>, Status> {
        self.dbg_print("check_if_integration_config_exists");

        let integration_id = request.into_inner().integration_id;

        let dbm = self.dbm.write().await;
        match dbm.check_if_integration_config_exists(integration_id).await {
            Ok(exists) => Ok(Response::new(
                get_check_if_integration_config_exists_response(exists),
            )),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn check_if_integration_config_online(
        &self,
        request: Request<CheckIfIntegrationConfigOnlineRequest>,
    ) -> Result<Response<CheckIfIntegrationConfigOnlineResponse>, Status> {
        self.dbg_print("check_if_integration_config_online");

        let integration_id = request.into_inner().integration_id;

        let dbm = self.dbm.write().await;
        match dbm.check_if_integration_config_online(integration_id).await {
            Ok(online) => Ok(Response::new(
                get_check_if_integration_config_online_response(online),
            )),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_integration_config(
        &self,
        request: Request<GetIntegrationConfigRequest>,
    ) -> Result<Response<GetIntegrationConfigResponse>, Status> {
        self.dbg_print("get_integration_config");

        let integration_id = request.into_inner().integration_id;

        let dbm = self.dbm.write().await;
        match dbm.get_integrations_config(integration_id).await {
            Ok(config) => Ok(Response::new(get_integration_config_response(config))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_integration_configs(
        &self,
        _: Request<GetAllIntegrationsRequest>,
    ) -> Result<Response<GetAllIntegrationsResponse>, Status> {
        self.dbg_print("get_all_integration_configs");

        let dbm = self.dbm.write().await;
        match dbm.get_integration_config().await {
            Ok(configs) => Ok(Response::new(get_all_integrations_response(configs))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_integration_configs_by_exchange(
        &self,
        request: Request<GetAllIntegrationsByExchangeRequest>,
    ) -> Result<Response<GetAllIntegrationsByExchangeResponse>, Status> {
        self.dbg_print("get_all_integration_configs_by_exchange");

        let exchange_id = request.into_inner().exchange_id;

        let dbm = self.dbm.write().await;
        match dbm
            .get_all_integration_configs_by_exchange(exchange_id)
            .await
        {
            Ok(configs) => Ok(Response::new(get_all_integrations_by_exchange_response(
                configs,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_online_integration_configs(
        &self,
        _: Request<GetAllOnlineIntegrationsRequest>,
    ) -> Result<Response<GetAllOnlineIntegrationsResponse>, Status> {
        self.dbg_print("get_all_online_integration_configs");

        let dbm = self.dbm.write().await;
        match dbm.get_all_online_integration_configs().await {
            Ok(configs) => Ok(Response::new(get_all_online_integrations_response(configs))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_all_offline_integration_configs(
        &self,
        _: Request<GetAllOfflineIntegrationsRequest>,
    ) -> Result<Response<GetAllOfflineIntegrationsResponse>, Status> {
        self.dbg_print("get_all_offline_integration_configs");

        let dbm = self.dbm.write().await;
        match dbm.get_all_offline_integration_configs().await {
            Ok(configs) => Ok(Response::new(get_all_offline_integrations_response(
                configs,
            ))),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn set_integration_online(
        &self,
        request: Request<SetIntegrationOnlineRequest>,
    ) -> Result<Response<SetIntegrationOnlineResponse>, Status> {
        self.dbg_print("set_integration_online");

        let integration_id = request.into_inner().integration_id;

        let dbm = self.dbm.write().await;
        match dbm.set_integration_online(integration_id).await {
            Ok(_) => Ok(Response::new(get_set_integration_online_response(
                true, None,
            ))),
            Err(e) => Ok(Response::new(get_set_integration_online_response(
                false,
                Some(e.to_string()),
            ))),
        }
    }

    async fn set_integration_offline(
        &self,
        request: Request<SetIntegrationOfflineRequest>,
    ) -> Result<Response<SetIntegrationOfflineResponse>, Status> {
        self.dbg_print("set_integration_offline");

        let integration_id = request.into_inner().integration_id;

        let dbm = self.dbm.write().await;
        match dbm.set_integration_offline(integration_id).await {
            Ok(_) => Ok(Response::new(get_set_integration_offline_response(
                true, None,
            ))),
            Err(e) => Ok(Response::new(get_set_integration_offline_response(
                false,
                Some(e.to_string()),
            ))),
        }
    }

    async fn update_integration_config(
        &self,
        request: Request<UpdateIntegrationRequest>,
    ) -> Result<Response<UpdateIntegrationResponse>, Status> {
        self.dbg_print("update_integration_config");

        let data = integration_config_from_proto(request.into_inner().integration.unwrap());

        let dbm = self.dbm.write().await;
        match dbm.update_integration_config(data).await {
            Ok(updated) => Ok(Response::new(get_update_integration_response(
                true,
                updated as u32,
                None,
            ))),
            Err(e) => Ok(Response::new(get_update_integration_response(
                false,
                0,
                Some(e.to_string()),
            ))),
        }
    }

    async fn delete_integration_config(
        &self,
        request: Request<DeleteIntegrationRequest>,
    ) -> Result<Response<DeleteIntegrationResponse>, Status> {
        self.dbg_print("delete_integration_config");

        let integration_id = request.into_inner().integration_id;

        let dbm = self.dbm.write().await;
        match dbm.delete_integration_config(integration_id).await {
            Ok(deleted) => Ok(Response::new(get_delete_integration_response(
                true,
                deleted as u32,
                None,
            ))),
            Err(e) => Ok(Response::new(get_delete_integration_response(
                false,
                0,
                Some(e.to_string()),
            ))),
        }
    }
}
