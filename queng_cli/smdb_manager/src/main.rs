use crate::svc_types::ServiceOP;
use smdb_provider::SMDBProvider;
use std::error::Error;

mod svc_handle;
mod svc_types;

pub const SPEC_OP: ServiceOP = ServiceOP::CheckIfAllServicesExists;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let host = "0.0.0.0".to_string();
    let port = 7070;

    let client = SMDBProvider::new(host, port).await;

    svc_handle::handle_service_op(&client, SPEC_OP)
        .await
        .expect("Failed to handle service");

    Ok(())
}
