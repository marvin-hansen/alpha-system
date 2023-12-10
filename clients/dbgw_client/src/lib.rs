use common::prelude::HostEndpoint;
use proto::binding::db_gateway_service_client::DbGatewayServiceClient as DBGWClient;
use std::error::Error;
use std::fmt;
use tonic::transport::{Channel, Uri};

mod cfg_gw;
mod svc_gw;

#[derive(Clone)]
pub struct DBGatewayClient {
    client: DBGWClient<Channel>,
}

impl DBGatewayClient {
    pub async fn new(config: HostEndpoint<'_>) -> Self {
        let port = config.port();
        let host = config.host_uri();

        // "http://[::1]:50051"
        let s = format!("http://{}:{}", host, port);
        let uri = s.parse::<Uri>()
            .expect(format!("DBGatewayClient: Failed to parse server URI: {}", s).as_str());

        // println!("DBGatewayClient: Server URI: {}", &s);

        // creating a channel that connects to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .expect(format!("\r\n DBGatewayClient: Failed to connect to DBGW service on: {} \r\n  \r\n Detail: \r\n", s).as_str());

        let client = DBGWClient::new(channel);

        Self { client }
    }
}

#[derive(Debug)]
pub struct DBGatewayError(pub String);

impl Error for DBGatewayError {}

impl fmt::Display for DBGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBGatewayError: {}", self.0)
    }
}
