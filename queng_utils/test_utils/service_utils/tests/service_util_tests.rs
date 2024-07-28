use std::env;
use std::time::Duration;

use common_config::prelude::ServiceID;
use service_utils::ServiceUtil;

fn setup_env() {
    // Set the environment variable.
    env::set_var("ENV", "LOCAL");
}
#[test]
fn test_start_service_util() {
    setup_env();

    let service_id = ServiceID::KaikoProxy;
    let svc_util = ServiceUtil::with_debug();

    let result = svc_util.start_service(&service_id);
    assert!(result.is_ok());

    std::thread::sleep(Duration::from_secs(25));

    let result = svc_util.stop_service(&service_id);
    assert!(result.is_ok());
}
