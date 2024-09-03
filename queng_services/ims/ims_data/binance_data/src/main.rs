mod handlers;
mod service;
mod stream_manager;

use crate::service::ImsDataServer;
use common_config::prelude::ServiceID;
use config_manager::CfgManager;
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
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
    let ctx_manager = CtxManager::new().await;
    let dns_manager = DnsManager::new(&ctx_manager).await;
    let cfg_manager = CfgManager::new(SVC_ID, svc_config, &ctx_manager, &dns_manager).await;

    // Create new gRPC server
    let grpc_svc = ImsDataServiceServer::new(ImsDataServer::new());

    // Run gRPC service
    grpc_service::start(DBG, SVC_ID, cfg_manager, grpc_svc).await
}
