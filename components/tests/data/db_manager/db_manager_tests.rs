use surrealdb::Error;

use common::prelude::{ServiceConfig, ServiceID};
use components::prelude::DBManager;
use specs::prelude::{cmdb_service_config, db_config_ci, smdb_service_config};

async fn get_dbm() -> Result<DBManager, Error> {
    let config = db_config_ci();
    let dbm = DBManager::new_offline(&config).await;
    Ok(dbm)
}

#[tokio::test]
async fn test_create_service() {
    let dbm = get_dbm().await.unwrap();

    let data = smdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);
}

#[tokio::test]
async fn test_check_if_service_id_exists() {
    let dbm = get_dbm().await.unwrap();

    // Doesn't exist
    let exists = dbm
        .check_if_service_id_exists(&ServiceID::SMDB)
        .await
        .unwrap();
    assert!(!exists);

    // Add service
    let data = smdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);

    // Does exist
    let exists = dbm
        .check_if_service_id_exists(&ServiceID::SMDB)
        .await
        .unwrap();
    assert!(exists);
}

#[tokio::test]
async fn test_check_if_services_exists() {
    let dbm = get_dbm().await.unwrap();
    let services: Vec<ServiceID> = vec![ServiceID::SMDB, ServiceID::CMDB];

    // Doesn't exist
    let exists = dbm.check_if_services_exists(&services).await.unwrap();
    assert!(!exists);

    // Add service
    let data = smdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);

    // Add service
    let data = cmdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);

    // Does exist
    let exists = dbm.check_if_services_exists(&services).await.unwrap();
    assert!(exists);
}

#[tokio::test]
async fn test_check_if_service_id_online() {
    let dbm = get_dbm().await.unwrap();
    // Doesn't exist
    let exists = dbm
        .check_if_service_id_exists(&ServiceID::SMDB)
        .await
        .unwrap();
    assert!(!exists);

    // Add service
    let data = smdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);

    // Does exist
    let exists = dbm
        .check_if_service_id_exists(&ServiceID::SMDB)
        .await
        .unwrap();
    assert!(exists);

    // Check if online
    let online = dbm
        .check_if_service_id_online(&ServiceID::SMDB)
        .await
        .unwrap();
    assert!(!online);
}

#[tokio::test]
async fn test_check_if_services_online() {
    let dbm = get_dbm().await.unwrap();
    let services: Vec<ServiceID> = vec![ServiceID::SMDB, ServiceID::CMDB];

    // Add service
    let data = smdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);

    // Add service
    let data = cmdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);

    // Check if all services online
    let online = dbm.check_if_services_online(&services).await.unwrap();
    assert!(!online);
}

#[tokio::test]
async fn test_read_all_services() {
    let dbm = get_dbm().await.unwrap();

    let data = smdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);

    let data = cmdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);

    let records: Vec<ServiceConfig> = dbm.read_all_services().await.unwrap();
    assert_eq!(records.len(), 2);
}

#[tokio::test]
async fn test_read_service_by_id() {
    let dbm = get_dbm().await.unwrap();

    let data = smdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);

    let record: Option<ServiceConfig> = dbm.read_record_by_id(&ServiceID::SMDB).await.unwrap();
    assert!(record.is_some());
}

#[tokio::test]
async fn test_set_service_online() {
    let dbm = get_dbm().await.unwrap();

    let data = smdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);

    // Check if online
    let online = dbm
        .check_if_service_id_online(&ServiceID::SMDB)
        .await
        .unwrap();
    assert!(!online);

    // set online
    let set_online = dbm.set_service_online(&ServiceID::SMDB).await.unwrap();
    assert!(set_online);

    // Check if online
    let online = dbm
        .check_if_service_id_online(&ServiceID::SMDB)
        .await
        .unwrap();
    assert!(online);
}

#[tokio::test]
async fn test_set_service_offline() {
    let dbm = get_dbm().await.unwrap();

    let data = smdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);

    // Check if not online
    let online = dbm
        .check_if_service_id_online(&ServiceID::SMDB)
        .await
        .unwrap();
    assert!(!online);

    // set online
    let set_online = dbm.set_service_online(&ServiceID::SMDB).await.unwrap();
    assert!(set_online);

    // Check if online
    let online = dbm
        .check_if_service_id_online(&ServiceID::SMDB)
        .await
        .unwrap();
    assert!(online);

    // set offline
    let set_offline = dbm.set_service_offline(&ServiceID::SMDB).await.unwrap();
    assert!(set_offline);

    // Check if not online
    let online = dbm
        .check_if_service_id_online(&ServiceID::SMDB)
        .await
        .unwrap();
    assert!(!online);
}

#[tokio::test]
async fn test_update_service() {
    let dbm = get_dbm().await.unwrap();

    let data = smdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);

    let data = smdb_service_config();
    let updated: Option<ServiceConfig> = dbm.update_service(data).await.unwrap();
    assert!(updated.is_some());
}

#[tokio::test]
async fn test_delete_service() {
    let dbm = get_dbm().await.unwrap();

    let data = smdb_service_config();
    let created = dbm.create_service(data).await.unwrap();
    assert!(created);

    let record: Option<ServiceConfig> = dbm.read_record_by_id(&ServiceID::SMDB).await.unwrap();
    assert!(record.is_some());

    let deleted = dbm.delete_service(&ServiceID::SMDB).await.unwrap();
    assert!(deleted);

    // Delete of non-existent data returns false
    let deleted = dbm.delete_service(&ServiceID::SMDB).await.unwrap();
    assert!(!deleted);
}
