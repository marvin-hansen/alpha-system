use components::prelude::DBManager;
use service_utils::shutdown;
use crate::SVC_ID;

// Sender is non-clonable therefore we need one for each server https://github.com/rust-lang/futures-rs/issues/1971
pub(crate) async fn http_sigint() {
    shutdown::wait_for_signal().await;
    println!("* http shutdown complete");
}

pub(crate) async fn grpc_sigint(dbm: DBManager) {
    shutdown::wait_for_signal().await;
    dbm.set_service_offline(&SVC_ID)
        .await
        .expect("!!Failed to set service offline!!");
    println!("* gRPC shutdown complete");
}
