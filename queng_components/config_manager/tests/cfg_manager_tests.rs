use common::prelude::{EnvironmentType, ServiceConfig, ServiceID};
use config_manager::CfgManager;
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
use std::env;

#[test]
fn new_config_manager_default() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(ctm.int_dns_server(), &Some("9.9.9.9".to_string()));

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");

    let config_manager = CfgManager::new(ServiceID::Default, &ctm, &dnm);

    assert_eq!(config_manager.get_svc_id(), ServiceID::Default);
    assert_eq!(config_manager.get_env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.get_svc_config(), ServiceConfig::default());
}
