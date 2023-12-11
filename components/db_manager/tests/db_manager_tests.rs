use surrealdb::Error;

use common::prelude::{AccountType, ExchangeID, PortfolioConfig, ServiceConfig, ServiceID};
use db_manager::DBManager;
use specs::prelude::{cmdb_service_config, db_config_ci, smdb_service_config};

async fn get_dbm() -> Result<DBManager, Error> {
    let config = db_config_ci();
    let dbm = DBManager::new_in_memory(&config).await;
    Ok(dbm)
}

fn get_portfolio_config() -> PortfolioConfig {
    let portfolio_id = 1;
    let portfolio_description = "cash portfolio".to_string();
    let portfolio_account_type = AccountType::Spot;
    let portfolio_account_id = "cash_account".to_string();
    let portfolio_exchange_id = ExchangeID::VEX;
    let portfolio_currency = "USD".to_string();
    let portfolio_cash_balance = 1000.0;
    let portfolio_max_drawdown = 20.0;
    let portfolio_instruments = vec!["BTC".to_string(), "ETH".to_string()];
    let instrument_max_allocation = 0.02;
    let instrument_max_drawdown = 10.0;

    PortfolioConfig::new_cash_portfolio(
        portfolio_id,
        portfolio_description,
        portfolio_account_type,
        portfolio_account_id,
        portfolio_exchange_id,
        portfolio_currency,
        portfolio_cash_balance,
        portfolio_max_drawdown,
        portfolio_instruments,
        instrument_max_allocation,
        instrument_max_drawdown,
    )
}

#[tokio::test]
async fn test_add_portfolio_config() {
    let dbm = get_dbm().await.unwrap();
    let config = get_portfolio_config();

    let result = dbm.add_portfolio_config(&config).await.unwrap();
    assert!(result);
}

#[tokio::test]
async fn test_read_all_portfolio_config() {
    let dbm = get_dbm().await.unwrap();
    let configs = dbm.read_all_portfolio_configs().await.unwrap();
    assert_eq!(configs.len(), 0);

    let config = get_portfolio_config();
    let result = dbm.add_portfolio_config(&config).await.unwrap();
    assert!(result);

    let configs = dbm.read_all_portfolio_configs().await.unwrap();
    assert_eq!(configs.len(), 1);
}

#[tokio::test]
async fn test_read_portfolio_config_by_id() {
    let dbm = get_dbm().await.unwrap();
    let config = get_portfolio_config();
    let result = dbm.add_portfolio_config(&config).await.unwrap();
    assert!(result);

    let config = dbm.read_portfolio_config_by_id(1).await.unwrap();
    assert_eq!(config.unwrap().portfolio_id(), 1);
}

#[tokio::test]
async fn test_update_portfolio_config() {
    let dbm = get_dbm().await.unwrap();
    let config = get_portfolio_config();
    let result = dbm.add_portfolio_config(&config).await.unwrap();
    assert!(result);

    let config = get_portfolio_config();
    let result = dbm.update_portfolio_config(config).await.unwrap();
    assert!(result.is_some());

    let config = dbm.read_portfolio_config_by_id(1).await.unwrap();
    assert_eq!(config.unwrap().portfolio_id(), 1);
}

#[tokio::test]
async fn test_delete_portfolio_config() {
    let dbm = get_dbm().await.unwrap();
    let delete_result = dbm.delete_portfolio_config(1).await.unwrap();
    assert!(!delete_result);

    let config = get_portfolio_config();
    let result = dbm.add_portfolio_config(&config).await.unwrap();
    assert!(result);

    let delete_result = dbm.delete_portfolio_config(1).await.unwrap();
    assert!(delete_result);
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
