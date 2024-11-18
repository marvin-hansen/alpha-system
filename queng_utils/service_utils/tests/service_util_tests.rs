use common_config::ServiceID;
use container_specs_postgres::postgres_db_container_config;
use docker_utils::DockerUtil;
use service_utils::ServiceUtil;
use service_utils::ServiceWaitStrategy;
use std::time::Duration;
use tokio::time::sleep;

// Somehow tests seem to be executed or sorted in alphabetical order,
// so make sure that the setup is on top of the stack.
#[tokio::test]
async fn all_setup() {
    let env = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres container
    let container_config = postgres_db_container_config();
    let result = env.get_or_start_container_config(&container_config);
    // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_service_util() {
    let res = ServiceUtil::with_debug().await;
    dbg!(&res);
    assert!(res.is_ok());
    let svc_util = res.unwrap();

    // Wait strategy
    let wait_strategy = ServiceWaitStrategy::Duration(Duration::from_millis(500));

    // Start DBGW service
    let service_id = ServiceID::DBGW;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    dbg!(&result);
    assert!(result.is_ok());

    // Start SMDB service - depends on DBGW
    let service_id = ServiceID::SMDB;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    assert!(result.is_ok());

    // Start CMDB service - depends on SMDB
    let service_id = ServiceID::CMDB;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    assert!(result.is_ok());

    // Starts  MDDB - depends on SMDB
    let service_id = ServiceID::MDDB;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    assert!(result.is_ok());

    // Starts  IMDB - depends on SMDB
    let service_id = ServiceID::IMDB;
    let result = svc_util.start_service(&service_id, &wait_strategy).await;
    assert!(result.is_ok());

    sleep(Duration::from_secs(1)).await;
}
