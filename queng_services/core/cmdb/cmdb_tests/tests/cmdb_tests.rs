use container_specs_postgres::postgres_db_container_config;
use docker_utils::prelude::DockerUtil;
use service_utils::ServiceUtil;

#[tokio::test]
async fn test_smdb() {
    let docker_util = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Start or reuse a test postgres database container
    let pg_container_config = postgres_db_container_config();
    let result = docker_util.get_or_start_container_config(&pg_container_config);
    dbg!(&result);
    assert!(result.is_ok());
    let (pg_container_id, _) = result.unwrap();

    // Start service util
    let res = ServiceUtil::with_debug().await;
    dbg!(&res);
    assert!(res.is_ok());
    // let svc_util = res.unwrap();
    // Get config manger for automatic configuration
    // let config_manager = svc_util.config_manager();

    // Stop and remove container
    let result = docker_util.stop_container(&pg_container_id);
    dbg!(&result);
    assert!(result.is_ok());
}
