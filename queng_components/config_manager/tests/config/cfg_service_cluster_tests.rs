use cmdb_specs::cmdb_service_config;
use common_config::prelude::ServiceID;
use common_env::prelude::EnvironmentType;
use config_manager::CfgManager;
use dbgw_specs::dbgw_service_config;
use smdb_specs::smdb_service_config;
use std::env;
// LOCAL and unknown environment cannot really be tested otherwise CI test runs breaks
// because the environment variable must be set in the CI environment (not in the test)
// Since you can onlu set the environment variable in the CI environment to one value,
// it was decided to test for the cluster environment as this is most critical.
// Please ensure the following is added to the CI test GH action:

//         env:
//           ENV: CLUSTER
//           DNS_SERVER: 9.9.9.9

#[tokio::test]
async fn test_init_smdb_env() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");
    // On a K8s cluster, PG_USER, PG_PASSWORD and PG_DATABASE usually are set as cluster secrets
    env::set_var("PG_USER", "postgres");
    env::set_var("PG_PASSWORD", "password");
    env::set_var("PG_DATABASE", "database");

    let service_id = ServiceID::SMDB;
    let cfg_manager = CfgManager::with_debug(service_id, smdb_service_config()).await;

    assert_eq!(cfg_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(cfg_manager.internal_dns_server(), "9.9.9.9:53");
    assert_eq!(cfg_manager.external_dns_server(), "1.1.1.1:53");
}

#[tokio::test]
async fn test_init_cmdb_env() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let service_id = ServiceID::CMDB;
    let cfg_manager = CfgManager::with_debug(service_id, cmdb_service_config()).await;

    assert_eq!(cfg_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(cfg_manager.internal_dns_server(), "9.9.9.9:53");
    assert_eq!(cfg_manager.external_dns_server(), "1.1.1.1:53");
}

#[tokio::test]
async fn test_init_dbgw_env() {
    env::set_var("ENV", "CLUSTER");
    env::set_var("DNS_SERVER", "9.9.9.9");

    let service_id = ServiceID::DBGW;
    let cfg_manager = CfgManager::with_debug(service_id, dbgw_service_config()).await;

    assert_eq!(cfg_manager.env_type(), EnvironmentType::CLUSTER);
    assert_eq!(cfg_manager.internal_dns_server(), "9.9.9.9:53");
    assert_eq!(cfg_manager.external_dns_server(), "1.1.1.1:53");
}
