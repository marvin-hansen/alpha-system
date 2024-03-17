mod service;

use crate::service::ImsDataServer;
use common::prelude::ServiceID;
use lib_data_stream;
use proto::binding::ims_data_service_server::ImsDataServiceServer;

const SVC_ID: ServiceID = ServiceID::ImsDataBinance;

#[tokio::main]
async fn main() {
    // Create new gRPC service
    let grpc_svc = ImsDataServiceServer::new(ImsDataServer::new());

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
