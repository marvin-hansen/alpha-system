use env_utils::EnvUtil;

#[tokio::test]
async fn test_env_util_teardown_container() {
    // Create new Env Utils
    let env_utils = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    // Get docker utils to run some checks
    let docker_util = &mut env_utils.docker_util();

    if !env_utils.all_containers_crated() {
        panic!("No containers crated thus nothing to teardown. Run setup_containers() to create all containers")
    }

    // extract the container names to check
    let api_proxy_container_name = &env_utils.api_proxy_container_name();
    let clickhouse_container_name = &env_utils.clickhouse_container_name();

    // Verify that the api proxy container was created
    let exists = docker_util
        .check_if_container_exists(&api_proxy_container_name)
        .expect("Failed to check if container exists");

    assert!(exists);
    println!("✅ OK: Container exits: {} ", api_proxy_container_name);
    println!();

    println!("✅ OK: Container exits: {} ", clickhouse_container_name);
    println!();

    println!("Removing all all containers");
    env_utils
        .teardown_all_containers()
        .await
        .expect("Failed to teardown containers");

    // Verify removal
    let exists = docker_util
        .check_if_container_exists(api_proxy_container_name)
        .expect("Failed to check if container exists");

    assert!(!exists);
    println!(
        "✅ OK: Container name: {} removed",
        api_proxy_container_name
    );
    println!();

    let exists = docker_util
        .check_if_container_exists(clickhouse_container_name)
        .expect("Failed to check if container exists");

    assert!(!exists);

    println!(
        "✅ OK: Container name: {} removed",
        clickhouse_container_name
    );
    println!();
}
