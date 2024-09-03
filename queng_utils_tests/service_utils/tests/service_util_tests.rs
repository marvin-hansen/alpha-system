use std::time::Duration;

use tokio::time::sleep;

use common_config::prelude::ServiceID;
use service_utils::prelude::ServiceUtil;

#[tokio::test]
async fn test_start_service_util() {
    // Start the service
    let service_id = ServiceID::DBGW;

    let res = ServiceUtil::with_debug().await;
    assert!(res.is_ok());
    let svc_util = res.unwrap();

    let result = svc_util.start_service(&service_id).await;
    assert!(result.is_ok());

    sleep(Duration::from_secs(1)).await;
}
