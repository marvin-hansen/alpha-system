use std::net::IpAddr;
use std::str::FromStr;

use tarpc::client;
use tarpc::tokio_serde::formats::Bincode;

use common::prelude::HostEndpoint;
use service::service::DBGatewayClient as DBGWClient;

mod svc_gw;

#[derive(Clone)]
pub struct DBGatewayClient {
    client: DBGWClient,
}

impl DBGatewayClient {
    pub async fn new(config: HostEndpoint<'_>) -> Self {
        let port = config.port();
        let host = config.host_uri();

        let ip_addr = IpAddr::from_str(host).expect("Failed to parse IP address from DBConfig");
        let server_addr = ((ip_addr), port);
        let codec_fn = Bincode::default;

        let mut transport = tarpc::serde_transport::tcp::connect(server_addr, codec_fn);
        transport.config_mut().max_frame_length(usize::MAX);

        let client = DBGWClient::new(client::Config::default(), transport.await.unwrap()).spawn();

        Self { client }
    }
}
