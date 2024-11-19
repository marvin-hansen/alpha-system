use common_config::HostEndpoint;
use proto_smdb::proto::db_gateway_smdb_service_client::DbGatewaySmdbServiceClient as DBGWClient;
use tonic::transport::{Channel, Uri};

mod client;
#[derive(Debug, Clone)]
pub struct DBGWSmdbClient {
    client: DBGWClient<Channel>,
}

impl DBGWSmdbClient {
    pub async fn new(config: HostEndpoint<'_>) -> Self {
        let port = config.port();
        let host = config.host_uri();

        let s = format!("http://{host}:{port}");
        let uri = s
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("DBGatewayClient: Failed to parse server URI: {s}"));

        Self::build(uri).await
    }

    pub async fn from_url(url: &str) -> Self {
        let uri = url
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("DBGatewayClient: Failed to parse server URI: {url}"));

        Self::build(uri).await
    }

    async fn build(uri: Uri) -> Self {
        // creating a channel that connects to server
        let channel = match Channel::builder(uri.clone()).connect().await {
            Ok(res) => res,
            Err(e) => {
                panic!("DBGatewayClient: Failed to connect to server: {uri} due to error: {e}");
            }
        };

        let client = DBGWClient::new(channel);

        Self { client }
    }
}
