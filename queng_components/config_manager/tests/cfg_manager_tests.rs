use common::prelude::{EnvironmentType, ServiceID};
use config_manager::CfgManager;
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
use smdb_specs::smdb_service_config;
use std::env;

#[test]
fn new_config_manager_default() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let ctm = CtxManager::new();
    assert_eq!(ctm.env_type(), EnvironmentType::CLUSTER);

    let dnm = DnsManager::new(&ctm);
    assert_eq!(dnm.internal_dns_server(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns_server(), "1.1.1.1:53");

    let config_manager = CfgManager::new(ServiceID::SMDB, smdb_service_config(), &ctm, &dnm);

    assert_eq!(config_manager.get_svc_id(), ServiceID::SMDB);
    assert_eq!(config_manager.get_env_type(), EnvironmentType::CLUSTER);
    assert_eq!(config_manager.get_svc_config(), smdb_service_config());
}
