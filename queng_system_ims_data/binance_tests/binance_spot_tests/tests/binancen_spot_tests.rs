use common_env::EnvironmentType;
use config_manager::CfgManager;
use service_utils::{ServiceStartConfig, ServiceUtil, WaitStrategy};

const ROOT_PATH: &str = "queng_system_ims_data/binance_tests/binance_spot_tests/tests";
const PROGRAM: &str = "ims_data_service";
const IGGY_HEALTH_URI: &str = "http://0.0.0.0:3000";
const IGGY_DARWIN_AARCH64: &str = "iggy_server_darwin_aarch64";
const IGGY_LINUX_X86_64: &str = "iggy_server_linux_x86_64";
const BINARIES: [&str; 3] = [PROGRAM, IGGY_DARWIN_AARCH64, IGGY_LINUX_X86_64];

fn select_iggy_binary(env: EnvironmentType) -> &'static str {
    match env {
        EnvironmentType::LOCAL => IGGY_DARWIN_AARCH64,
        EnvironmentType::CI => IGGY_LINUX_X86_64,
        _ => panic!("Unsupported environment"),
    }
}

fn get_service_start_config(program: &'static str, url: String) -> ServiceStartConfig {
    ServiceStartConfig::builder()
        .program(program)
        .wait_strategy(WaitStrategy::WaitForHttpHealthCheck(url, 5))
        .build()
}

#[tokio::test]
async fn test_binance_spot() {
    dbg!("Start service util");
    let res = ServiceUtil::with_debug(ROOT_PATH, Vec::from(BINARIES)).await;
    if res.is_err() {
        dbg!(&res);
    }
    assert!(res.is_ok());
    let svc_util = res.unwrap();
    dbg!("✅ service util started");

    dbg!("Start config manager");
    let config_manager = CfgManager::default_with_debug();
    dbg!("✅ config manager started");

    dbg!("Detect Environment");
    let env = config_manager.env_type();
    dbg!(&format!("✅ Detected Environment: {}", env));

    dbg!("Configure iggy messaging service");
    let binary = select_iggy_binary(env);
    let uri = IGGY_HEALTH_URI;
    let iggy_start_config = get_service_start_config(binary, uri.to_string());

    dbg!("Start iggy messaging service");
    let result = svc_util.start_service_from_config(iggy_start_config).await;
    if result.is_err() {
        dbg!(&result);
    }
    assert!(result.is_ok());
    dbg!("✅ iggy messaging service started");

    dbg!("Configure IMS Data service - Binance Spot");
    let (uri, _) = config_manager
        .get_metrics_socket_addr_uri()
        .expect("Failed to get host and port for IMS Data service");

    dbg!(&format!(" IMS Data service uri: {uri}"));

    dbg!("Configure IMS Data service - Binance Spot");
    let dbgw_start_config = get_service_start_config(PROGRAM, uri);

    dbg!("Start IMS Data service - Binance Spot");
    let result = svc_util.start_service_from_config(dbgw_start_config).await;
    if result.is_err() {
        dbg!(&result);
    }
    assert!(result.is_ok());
    dbg!("✅ IMS Data service service started");
}
