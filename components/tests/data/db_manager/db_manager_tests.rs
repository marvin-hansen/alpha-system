use surrealdb::engine::local;
use surrealdb::Error;
use surrealdb::Surreal;

use common::prelude::{ServiceConfig, ServiceID};
use components::prelude::DBManager;
use specs::prelude::{cmdb_service_config, smdb_service_config};

async fn get_dbm() -> Result<DBManager, Error> {
    let db: Surreal<local::Db> = Surreal::new::<local::Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    let dbm = DBManager::new(db);

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
