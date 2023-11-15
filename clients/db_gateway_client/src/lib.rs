use std::net::IpAddr;
use std::str::FromStr;

use tarpc::{client, context};
use tarpc::tokio_serde::formats::Bincode;

use common::prelude::{DBConfig, DBGatewayError, ServiceConfig, ServiceID};
use service::DBGatewayClient as DBGWClient;

pub struct DBGatewayClient {
    client: DBGWClient,
}

impl DBGatewayClient {
    pub async fn new(config: DBConfig) -> Self {
        let port = config.port();
        let host = config
            .host()
            .clone()
            .expect("Failed to get host from DBConfig");

        let ip_addr = IpAddr::from_str(&host).expect("Failed to parse IP address from DBConfig");
        let server_addr = ((ip_addr), port);
        let codec_fn = Bincode::default;

        let mut transport = tarpc::serde_transport::tcp::connect(server_addr, codec_fn);
        transport.config_mut().max_frame_length(usize::MAX);

        let client = DBGWClient::new(client::Config::default(), transport.await.unwrap()).spawn();

        Self { client }
    }
}

impl DBGatewayClient {
    pub async fn create_service(&self, data: ServiceConfig) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .create_service(context::current(), data)
            .await
            .expect("RPC call failed to create service");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn read_all_services(&self) -> Result<Vec<ServiceConfig>, DBGatewayError> {
        let res = self
            .client
            .read_all_services(context::current())
            .await
            .expect("RPC call failed to read all services");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn read_service_by_id(&self, id: ServiceID) -> Result<ServiceConfig, DBGatewayError> {
        let res = self
            .client
            .read_service_by_id(context::current(), id)
            .await
            .expect("RPC call failed to read_service_by_id");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn update_service(&self, data: ServiceConfig) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .update_service(context::current(), data)
            .await
            .expect("RPC call failed to update service");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_service(&self, id: ServiceID) -> Result<bool, DBGatewayError> {
        let res = self
            .client
            .delete_service(context::current(), id)
            .await
            .expect("RPC call failed to delete service");
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }
}
