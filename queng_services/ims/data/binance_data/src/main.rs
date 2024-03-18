mod client;
mod handlers;
mod service;
mod stream_manager;
mod types;

use crate::service::ImsDataServer;
use common::prelude::ServiceID;
use lib_data_stream;
use proto::binding::ims_data_service_server::ImsDataServiceServer;

const SVC_ID: ServiceID = ServiceID::ImsDataBinance;

#[tokio::main]
async fn main() {
    // Create new data stream service
    let svc = ImsDataServer::new();
    // Download reference symbols from Binance
    svc.update_reference_symbols()
        .await
        .expect("Failed to update reference symbols");
    // Create new gRPC server
    let grpc_svc = ImsDataServiceServer::new(svc);

    // Create & configure health service
    let (mut health_reporter, health_svc) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<ImsDataServiceServer<ImsDataServer>>()
        .await;

    // Run gRPC service
    lib_data_stream::run(SVC_ID, grpc_svc, health_svc)
        .await
        .expect("Failed to start data stream service for Binance ")
}
