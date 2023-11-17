use common::prelude::HostEndpoint;
use smdb_client::SMDBClient;

use crate::svc_types::ServiceOP;

mod svc_types;
mod svc_handle;

pub const SPEC_OP: ServiceOP = ServiceOP::CheckIfServiceIDExists;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let host_endpoint = HostEndpoint::new("127.0.0.1", 7070);
    let client = SMDBClient::new(host_endpoint).await;

    svc_handle::handle_service_op(&client, SPEC_OP)
        .await
        .expect("Failed to handle service");

    Ok(())
}

