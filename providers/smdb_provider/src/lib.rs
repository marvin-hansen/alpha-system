use std::net::IpAddr;
use std::str::FromStr;

use tarpc::client;
use tarpc::tokio_serde::formats::Bincode;

use common::prelude::{HostEndpoint, ServiceID};
use components::prelude::ServiceManager;
use smdb_service::service::SMDBServiceClient;

mod prv_smdb;

#[derive(Clone)]
pub struct SMDBProvider {
    client: SMDBServiceClient,
}

impl SMDBProvider {
    /// new creates a new SMDBProvider, finds the host and port of the SMDB service,
    /// and configures the smdb client fully automatically relative to the detected context.
    pub async fn new(svm: &ServiceManager<'_>) -> Self {
        // SvcEnvManager configures SMDB ip and port relative to the detected context.
        let (host, port) = svm
            .get_service_host_port(ServiceID::SMDB)
            .expect("[SMDBProvider]: Failed to get host and port for: SMDB");

        let ip_addr = IpAddr::from_str(&host).expect("Failed to parse IP address from DBConfig");
        let server_addr = ((ip_addr), port);
        let codec_fn = Bincode::default;

        let mut transport = tarpc::serde_transport::tcp::connect(server_addr, codec_fn);
        transport.config_mut().max_frame_length(usize::MAX);

        let client =
            SMDBServiceClient::new(client::Config::default(), transport.await.unwrap()).spawn();

        Self { client }
    }

    /// from_host_endpoint creates a new SMDBProvider from a host endpoint.
    /// Use this for CLI applications or offline testing without autoconfiguration.
    pub async fn from_host_endpoint(config: HostEndpoint<'_>) -> Self {
        let port = config.port();
        let host = config.host_uri();

        let ip_addr = IpAddr::from_str(host).expect("Failed to parse IP address from DBConfig");
        let server_addr = ((ip_addr), port);
        let codec_fn = Bincode::default;

        let mut transport = tarpc::serde_transport::tcp::connect(server_addr, codec_fn);
        transport.config_mut().max_frame_length(usize::MAX);

        let client =
            SMDBServiceClient::new(client::Config::default(), transport.await.unwrap()).spawn();

        Self { client }
    }
}
