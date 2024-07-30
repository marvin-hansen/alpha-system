use std::time::Duration;

use tokio::time::sleep;

use common_config::prelude::ServiceID;
use env_utils::EnvUtil;
use service_utils::prelude::ServiceUtil;

async fn setup_env() {
    // dbgw require postgres
    let env_util = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");
    env_util
        .setup_postgres()
        .await
        .expect("Failed to setup postgres");
}

#[tokio::test]
async fn test_start_service_util() {
    // setup_env().await;

    // Start the service
    let service_id = ServiceID::KaikoProxy;

    let res = ServiceUtil::with_debug();
    assert!(res.is_ok());
    let svc_util = res.unwrap();

    let result = svc_util.start_service(&service_id);
    assert!(result.is_ok());

    sleep(Duration::from_secs(1)).await;
}
