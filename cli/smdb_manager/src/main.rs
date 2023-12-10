use crate::svc_types::ServiceOP;
use common::prelude::HostEndpoint;
use smdb_provider::SMDBProvider;

mod svc_handle;
mod svc_types;

pub const SPEC_OP: ServiceOP = ServiceOP::CheckIfServicesExists;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let endpoint = HostEndpoint::new("127.0.0.1", 7070);
    let client = SMDBProvider::from_host_endpoint(endpoint).await;

    svc_handle::handle_service_op(&client, SPEC_OP)
        .await
        .expect("Failed to handle service");

    Ok(())
}
