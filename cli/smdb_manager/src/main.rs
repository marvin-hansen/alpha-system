use crate::svc_types::ServiceOP;
use smdb_client::SMDBClient;

mod svc_handle;
mod svc_types;

pub const SPEC_OP: ServiceOP = ServiceOP::CheckIfServiceIDExists;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = SMDBClient::new("127.0.0.1", 7070).await;
    svc_handle::handle_service_op(&client, SPEC_OP)
        .await
        .expect("Failed to handle service");

    Ok(())
}
