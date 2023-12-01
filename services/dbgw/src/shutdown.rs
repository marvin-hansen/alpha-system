use crate::SVC_ID;
use components::prelude::DBManager;
use tokio::signal::unix::{signal, SignalKind};

// Signal sender is non-clonable therefore we need to create a new one for each server.
// https://github.com/rust-lang/futures-rs/issues/1971
pub async fn http_sigint() {
    let _ = signal(SignalKind::terminate())
        .expect("failed to create a new SIGINT signal handler for htttp")
        .recv()
        .await;
    println!("* Http shutdown complete");
}

pub async fn grpc_sigint(dbm: DBManager) {
    let _ = signal(SignalKind::terminate())
        .expect("failed to create a new SIGINT signal handler for gRPC")
        .recv()
        .await;

    // Set service to offline
    dbm.set_service_offline(&SVC_ID)
        .await
        .expect("Failed to close database connection");

    println!("* gRPC shutdown complete");
}
