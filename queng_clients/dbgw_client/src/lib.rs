use common::prelude::HostEndpoint;
use proto::binding::db_gateway_service_client::DbGatewayServiceClient as DBGWClient;
use tonic::transport::{Channel, Uri};

mod cfg_gw;
mod error;
mod svc_gw;

#[derive(Debug, Clone)]
pub struct DBGatewayClient {
    client: DBGWClient<Channel>,
}

impl DBGatewayClient {
    pub async fn new(config: HostEndpoint<'_>) -> Self {
        let port = config.port();
        let host = config.host_uri();

        // "http://[::1]:50051"
        let s = format!("http://{}:{}", host, port);
        let uri = s
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("DBGatewayClient: Failed to parse server URI: {}", s));

        // println!("DBGatewayClient: Server URI: {}", &s);

        // creating a channel that connects to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .unwrap_or_else(|_| panic!("\r\n DBGatewayClient: Failed to connect to DBGW service on: {} \r\n  \r\n Detail: \r\n", s));

        let client = DBGWClient::new(channel);

        Self { client }
    }
}
