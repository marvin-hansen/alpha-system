use mimalloc::MiMalloc;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;

use common_config::prelude::ServiceID;
use config_manager::CfgManager;
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
use pg_cmdb_manager::PostgresCMDBManager;
use proto_cmdb::proto::cmdb_service_server::CmdbServiceServer;

use crate::service::CMDBServer;

mod service;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: ServiceID = ServiceID::CMDB;
const DBG: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup autoconfiguration.
    let svc_config = cmdb_specs::cmdb_service_config();
    let ctx_manager = CtxManager::new().await;
    let dns_manager = DnsManager::new(&ctx_manager).await;
    let cfg_manager = CfgManager::new(SVC_ID, svc_config, &ctx_manager, &dns_manager).await;

    // Configure DB Manager
    let pg_config = cfg_manager.postgres_db_config();
    let dbm = PostgresCMDBManager::new(&pg_config.pg_connection_url())
        .await
        .expect("Failed to create DB Manager");

    // Construct gRPC service
    let arc_dbm = Arc::new(RwLock::new(dbm));
    let grpc_svc = CmdbServiceServer::new(CMDBServer::new(arc_dbm));

    // Run gRPC service
    lib_data_stream::run(DBG, SVC_ID, cfg_manager, grpc_svc).await
}
