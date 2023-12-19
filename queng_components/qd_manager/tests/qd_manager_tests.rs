use cfg_manager::CfgManager;
use common::prelude::{FileConfigType, ServiceID};
use ctx_manager::CtxManager;
use qd_manager::QDManager;
use std::env;
#[test]
fn test_get_data_bars() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "175.24.54.1");

    let ctx = CtxManager::new();
    let cfg_manager = CfgManager::new(ServiceID::Default, &ctx);

    let qd_manager = QDManager::new(&cfg_manager);

    let symbol = &FileConfigType::BtcSmall;

    let result = qd_manager.get_data_bars(symbol);

    assert!(result.is_ok());

    let bars = result.expect("get data bars failed");
    assert_eq!(bars.len(), 1000);
}
