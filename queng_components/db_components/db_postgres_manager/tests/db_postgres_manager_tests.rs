use std::env;

use common_config::prelude::ServiceID;
use db_postgres_manager::PostgresDBManager;
use env_utils::EnvUtil;

async fn setup_ci_env() {
    env::set_var("ENV", "CI");

    let env_util = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    env_util
        .setup_postgres()
        .await
        .expect("Failed to setup postgres");
}

#[tokio::test]
async fn test_db_postgres_manager() {
    setup_ci_env().await;

    let pg_config = db_specs::postgres_db::get_ci_db_config();

    let pgm = PostgresDBManager::with_debug(&pg_config)
        .await
        .expect("Failed to get PostgresDBManager");

    test_count_services(&pgm).await;
    test_check_if_service_id_exists(&pgm).await;
    test_check_if_service_id_online(&pgm).await;
    test_check_if_services_online(&pgm).await;
    test_set_service_online(&pgm).await;
    test_set_service_offline(&pgm).await;
    // test_read_service_by_id(&pgm).await;
    // test_read_all_services(&pgm).await;
    // test_update_service(&pgm).await;

    pgm.close().await
}

async fn test_count_services(pgm: &PostgresDBManager) {
    let res = pgm.count_services().await;
    assert!(res.is_ok());

    let count = res.unwrap();
    assert!(count > 0);
}

async fn test_check_if_service_id_exists(pgm: &PostgresDBManager) {
    let svc_id = ServiceID::SMDB;
    let res = pgm.check_if_service_id_exists(&svc_id).await;
    assert!(res.is_ok());

    let exists = res.unwrap();
    assert!(exists);
}

async fn test_check_if_service_id_online(pgm: &PostgresDBManager) {
    let svc_id = ServiceID::SMDB;
    let res = pgm.check_if_service_id_online(&svc_id).await;
    assert!(res.is_ok());

    let online = res.unwrap();
    // The test data is not set to online
    assert!(!online);
}

async fn test_check_if_services_online(pgm: &PostgresDBManager) {
    let services = Vec::from([ServiceID::SMDB, ServiceID::CMDB]);
    let res = pgm.check_if_services_online(&services).await;
    assert!(res.is_ok());

    let online = res.unwrap();
    // The test data is not set to online
    assert!(!online);
}

async fn test_set_service_online(pgm: &PostgresDBManager) {
    let svc_id = ServiceID::SMDB;
    let res = pgm.check_if_service_id_online(&svc_id).await;
    assert!(res.is_ok());

    // The test data is not set to online
    let online = res.unwrap();
    assert!(!online);

    // Set the service to online
    let res = pgm.set_service_online(&svc_id).await;
    assert!(res.is_ok());

    // Check that the service is online now
    let res = pgm.check_if_service_id_online(&svc_id).await;
    assert!(res.is_ok());

    let online = res.unwrap();
    assert!(online);
}

async fn test_set_service_offline(pgm: &PostgresDBManager) {
    let svc_id = ServiceID::SMDB;

    // Check that the service is online
    let res = pgm.check_if_service_id_online(&svc_id).await;
    assert!(res.is_ok());

    let online = res.unwrap();
    assert!(online);

    // Set the service to offline
    let res = pgm.set_service_offline(&svc_id).await;
    assert!(res.is_ok());

    // Check that the service is offline again
    let res = pgm.check_if_service_id_online(&svc_id).await;
    assert!(res.is_ok());

    let online = res.unwrap();
    assert!(!online);
}

async fn _test_read_service_by_id(pgm: &PostgresDBManager) {
    let svc_id = ServiceID::SMDB;
    let res = pgm.read_service_by_id(&svc_id).await;
    assert!(res.is_ok());
    let opt_svc = res.unwrap();
    assert!(opt_svc.is_some());
    let svc = opt_svc.unwrap();
    assert_eq!(svc.svc_id(), &svc_id);
}

async fn _test_read_all_services(pgm: &PostgresDBManager) {
    let res = pgm.read_all_services().await;
    assert!(res.is_ok());
    let services = res.unwrap();
    assert!(!services.is_empty());
}

async fn _test_update_service(pgm: &PostgresDBManager) {
    let svc_id = ServiceID::SMDB;
    let res = pgm.read_service_by_id(&svc_id).await;
    assert!(res.is_ok());
    let opt_svc = res.unwrap();
    assert!(opt_svc.is_some());
    let svc = opt_svc.unwrap();

    let res = pgm.update_service(svc).await;
    assert!(res.is_ok());
}
