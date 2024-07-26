use common_config::prelude::ServiceID;
use dbgw_client::DBGatewayClient;
use env_utils::EnvUtil;

async fn setup_ci_env() {
    let env_util = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    env_util
        .setup_postgres()
        .await
        .expect("Failed to setup postgres");
}

// Add proper integration test that starts DBGW for the test
// https://github.com/dzbarsky/rules_itest/blob/master/tests/test_env/BUILD.bazel

// Run this test with
// bazel test //... --test_tag_filters=core_services_tests  --test_env=ENV=LOCAL

#[tokio::test]
async fn test_core_services() {
    setup_ci_env().await;

    let url = "http://127.0.0.1:9090";
    let dbgw_client = DBGatewayClient::from_url(url).await;

    let res = dbgw_client.read_service_by_id(ServiceID::DBGW).await;

    assert!(res.is_ok());
    let svc = res.unwrap().unwrap();
    assert_eq!(svc.svc_id(), &ServiceID::DBGW);
}
