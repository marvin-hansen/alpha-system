use env_utils::EnvUtil;

#[tokio::test]
async fn test_env_util_teardown_ci() {
    // Initial setup of the CI test environment
    let ci_env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    // Call teardown to delete DB w/o deleting the container
    let res = ci_env.teardown_ci().await;

    assert!(res.is_ok())
}
