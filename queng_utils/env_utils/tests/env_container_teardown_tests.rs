use env_utils::EnvUtil;

#[tokio::test]
async fn test_env_util_teardown_container() {
    // Create new Env Utils
    let ci_env = EnvUtil::with_debug();

    // Initial setup of the CI containers
    ci_env
        .teardown_containers()
        .await
        .expect("Failed to teardown containers");

    // get docker utils to run some checks
    let docker_util = &mut ci_env.get_docker_util().expect("Failed to get docker util");

    // Verify that the api proxy container was created
    let api_proxy_container_name = ci_env.api_proxy_container_name();
    let exists = docker_util
        .check_if_container_exists(&api_proxy_container_name)
        .expect("Failed to check if container exists");

    assert_eq!(exists, false);

    println!(
        "✅ OK: Container name: {} removed",
        api_proxy_container_name
    );
    println!();

    // Verify that the clickhouse container was created
    let clickhouse_container_name = ci_env.clickhouse_container_name();
    let exists = docker_util
        .check_if_container_exists(&clickhouse_container_name)
        .expect("Failed to check if container exists");

    assert_eq!(exists, false);

    println!(
        "✅ OK: Container name: {} removed",
        clickhouse_container_name
    );
    println!();
}
