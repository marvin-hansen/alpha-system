use env_utils::EnvUtil;

#[tokio::test]
async fn test_setup() {
    let env = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    let result = env.setup_container_postgres_db().await;
    assert!(result.is_ok());
}
