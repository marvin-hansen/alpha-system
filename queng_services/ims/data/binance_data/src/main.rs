mod handlers;
mod service;
mod stream_manager;
mod types;

use crate::service::ImsDataServer;
use common::prelude::ServiceID;
use lib_data_stream;
use proto_bindings::proto::ims_data_service_server::ImsDataServiceServer;

const SVC_ID: ServiceID = ServiceID::ImsDataBinance;

#[tokio::main]
async fn main() {
    // Create new gRPC server
    let grpc_svc = ImsDataServiceServer::new(ImsDataServer::new());

    // Run gRPC service
    lib_data_stream::run(SVC_ID, grpc_svc)
        .await
        .expect("Failed to start data stream service for Binance ")
}
