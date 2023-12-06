use common::prelude::HostEndpoint;
use proto_binding::dbgw::db_gateway_service_client::DbGatewayServiceClient as DBGWClient;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use tonic::transport::{Channel, Uri};

mod cfg_gw;
mod svc_gw;

#[derive(Clone)]
pub struct DBGatewayClient {
    client: RefCell<DBGWClient<Channel>>,
}

impl DBGatewayClient {
    pub async fn new(config: HostEndpoint<'_>) -> Self {
        let port = config.port();
        let host = config.host_uri();

        // "http://[::1]:50051"
        let s = format!("http://{}:{}", host, port);
        let uri = s.parse::<Uri>().unwrap();
        println!("Server URI: {}", &s);

        // creating a channel ie connection to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .expect("Failed to connect to server");

        let client = DBGWClient::new(channel);

        Self {
            client: RefCell::new(client),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBGatewayError(pub String);

impl Error for DBGatewayError {}

impl fmt::Display for DBGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBGatewayError: {}", self.0)
    }
}
