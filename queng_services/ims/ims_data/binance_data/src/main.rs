mod handlers;
mod service;
mod stream_manager;

use crate::service::ImsDataServer;
use common_config::prelude::ServiceID;
use mimalloc::MiMalloc;
use proto_imdb::proto::ims_data_service_server::ImsDataServiceServer;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: ServiceID = ServiceID::ImsDataBinance;

#[tokio::main]
async fn main() {
    // Create new configuration
    let svc_config = ims_data_binance_specs::ims_data_binance_config();

    // Create new gRPC server
    let grpc_svc = ImsDataServiceServer::new(ImsDataServer::new());

    // Run gRPC service
    lib_data_stream::run(SVC_ID, svc_config, grpc_svc)
        .await
        .expect("Failed to start data stream service for Binance ")
}
