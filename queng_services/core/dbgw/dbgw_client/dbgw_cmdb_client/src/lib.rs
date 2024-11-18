use common_config::HostEndpoint;
use proto_cmdb::proto::db_gateway_cmdb_service_client::DbGatewayCmdbServiceClient as DBGWClient;
use tonic::transport::{Channel, Uri};

mod client;

#[derive(Debug, Clone)]
pub struct DBGWCmdbClient {
    client: DBGWClient<Channel>,
}

impl DBGWCmdbClient {
    pub async fn new(config: HostEndpoint<'_>) -> Self {
        let s = format!("http://{}:{}", config.host_uri(), config.port());
        let uri = s
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("DBGWCmdbClient: Failed to parse server URI: {}", s));

        Self::build(uri).await
    }

    pub async fn from_url(url: &str) -> Self {
        let uri = url
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("DBGWCmdbClient: Failed to parse server URI: {}", url));

        Self::build(uri).await
    }

    async fn build(uri: Uri) -> Self {
        let channel = match Channel::builder(uri.clone()).connect().await {
            Ok(res) => res,
            Err(e) => {
                panic!(
                    "DBGWCmdbClient: Failed to connect to DBGW server: {} due to error: {}",
                    uri, e
                );
            }
        };

        Self {
            client: DBGWClient::new(channel),
        }
    }
}
