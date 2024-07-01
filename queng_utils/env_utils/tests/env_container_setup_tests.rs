use env_utils::EnvUtil;

#[tokio::test]
async fn test_env_util_setup_containers() {
    // Create new Env Utils
    let mut ci_env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    // Initial setup of the CI containers
    ci_env
        .setup_containers()
        .await
        .expect("Failed to setup ci containers");

    // get docker utils to run some checks
    let docker_util = &mut ci_env.docker_util();

    // Verify that the api proxy container was created
    let api_proxy_container_name = ci_env.api_proxy_container_name();
    let exists = docker_util
        .check_if_container_exists(api_proxy_container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    println!(
        "✅ OK: Container name: {} created",
        api_proxy_container_name
    );
    println!();

    // Verify that the clickhouse container was created
    let clickhouse_container_name = ci_env.clickhouse_container_name();
    let exists = docker_util
        .check_if_container_exists(clickhouse_container_name)
        .expect("Failed to check if container exists");
    assert!(exists);

    println!(
        "✅ OK: Container name: {} created",
        clickhouse_container_name
    );
    println!();
}
