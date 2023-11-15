use common::prelude::HostEndpoint;
use db_gateway_client::DBGatewayClient;

use crate::types::{ServiceOP, SpecType};
use crate::types::SpecType::ServiceConfig;

mod types;
mod svc_handle;

pub const SPEC_TYPE: SpecType = ServiceConfig;
pub const SPEC_OP: ServiceOP = ServiceOP::CreateAllService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let host_endpoint = HostEndpoint::new("127.0.0.1", 7687);
    let client = DBGatewayClient::new(host_endpoint).await;

    match SPEC_TYPE {
        ServiceConfig => {
            svc_handle::handle_service_op(&client, SPEC_OP)
                .await
                .expect("Failed to handle service op");
        }
    }

    Ok(())
}
