// Unsafe code must be explicitly enabled to use it.
#[deny(unsafe_code)]
//
mod handlers;
mod service;
mod stream_manager;

use crate::service::ImsDataServer;
use common_config::prelude::ServiceID;
use config_manager::CfgManager;

use grpc_service;
use mimalloc::MiMalloc;
use proto_imdb::proto::ims_data_service_server::ImsDataServiceServer;
use std::error::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: ServiceID = ServiceID::ImsDataBinance;
const DBG: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create auto configuration
    let svc_config = ims_data_binance_specs::ims_data_binance_config();

    let cfg_manager = CfgManager::new(SVC_ID, svc_config).await;

    // Create new gRPC server
    let grpc_svc = ImsDataServiceServer::new(ImsDataServer::new());

    // Run gRPC service
    grpc_service::start(DBG, SVC_ID, cfg_manager, grpc_svc).await
}
