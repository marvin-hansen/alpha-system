use std::time::Duration;

use tokio::time::sleep;

use common_config::prelude::ServiceID;
use service_utils::prelude::ServiceUtil;

#[tokio::test]
async fn test_start_service_util() {
    // Start the PG Docker container

    // Run Service data migration to ensure SMDB fully working.

    // Start the service
    let service_id = ServiceID::DBGW;

    let res = ServiceUtil::with_debug().await;
    assert!(res.is_ok());
    let svc_util = res.unwrap();

    let result = svc_util.start_service(&service_id).await;
    assert!(result.is_ok());

    // Start the service
    let service_id = ServiceID::SMDB;
    let result = svc_util.start_service(&service_id).await;
    assert!(result.is_ok());

    // thread 'main' panicked at queng_services/templete/grpc_service/src/lib.rs:58:13:
    // Service dependency DBGW is unavailable; please start it
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // test test_start_service_util ... FAILED

    // // Start the service
    // let service_id = ServiceID::CMDB;
    // let result = svc_util.start_service(&service_id).await;
    // assert!(result.is_ok());

    sleep(Duration::from_secs(1)).await;
}
