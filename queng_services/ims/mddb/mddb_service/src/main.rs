use std::error::Error;

use mimalloc::MiMalloc;

use common_config::prelude::ServiceID;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: ServiceID = ServiceID::MDDB;
// const DBG: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup autoconfiguration.
    // let svc_config = mddb_specs::mddb_service_config();
    // let cfg_manager = CfgManager::new(SVC_ID, svc_config).await;

    eprintln!("Starting service: {}", SVC_ID);

    Ok(())

    // Run gRPC service
    // grpc_service::start(DBG, SVC_ID, cfg_manager, grpc_svc).await
}
