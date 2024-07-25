use std::time::Duration;

use common_config::prelude::ServiceID;
use service_utils::ServiceUtil;

#[test]
fn test_start_service_util() {
    let service_id = ServiceID::KaikoProxy;
    let svc_util = ServiceUtil::with_debug();

    let result = svc_util.start_service(&service_id);
    assert!(result.is_ok());

    std::thread::sleep(Duration::from_secs(5));

    let result = svc_util.stop_service(&service_id);
    assert!(result.is_ok());
}
