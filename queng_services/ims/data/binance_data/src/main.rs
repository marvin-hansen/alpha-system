mod handlers;
mod service;
mod stream_manager;
mod types;

use crate::service::ImsDataServer;
use common::prelude::ServiceID;
use ims_data_binance_specs;
use lib_data_stream;
use proto_bindings::proto::ims_data_service_server::ImsDataServiceServer;

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
