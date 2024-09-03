use std::error::Error;
use std::sync::{Arc, RwLock};

use mimalloc::MiMalloc;

use common_config::prelude::ServiceID;
use config_manager::CfgManager;
use ctx_manager::CtxManager;
use db_clickhouse_manager::ClickhouseDBManager;
use dns_manager::DnsManager;
use proto_mddb::proto::mdm_service_server::MdmServiceServer;
use symbol_manager::SymbolManager;

use crate::service::MDMServer;

mod service;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: ServiceID = ServiceID::MDDB;
const DBG: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup autoconfiguration.
    let svc_config = mddb_specs::mddb_service_config();
    let ctx_manager = CtxManager::new().await;
    let dns_manager = DnsManager::new(&ctx_manager).await;
    let cfg_manager = CfgManager::new(SVC_ID, svc_config, &ctx_manager, &dns_manager).await;

    // println!("[MDDB]: Create a new QueryDBManager instance.");
    let db_config = cfg_manager.clickhouse_db_config().to_owned();
    let mut q_manager = ClickhouseDBManager::new(db_config)
        .await
        .expect("[MDDB]/main: Failed to create QueryDBManager instance.");

    // println!("[MDDB]: Get the symbol table for the default exchange.");
    let default_exchange = cfg_manager.default_exchange();
    let exchange_symbol_table = cfg_manager
        .get_symbol_table(default_exchange)
        .expect("[MDDB]/main: Failed to get symbol table for default exchange.");

    // println!("[MDDB]: Get all symbols for the default exchange.");
    let symbols = q_manager
        .get_all_symbols_with_ids(&exchange_symbol_table)
        .await
        .expect("[MDDB]/main: Failed to get all symbols for SymbolManager.");

    let exchanges = cfg_manager.exchanges_id_names().to_owned();

    // println!("[MDDB]: Create a new SymbolManager instance.");
    let symbol_manager = async {
        Arc::new(RwLock::new(
            SymbolManager::new(symbols, exchanges)
                .expect("[MDDB]/main: Failed to create SymbolManager instance."),
        ))
    }
    .await;

    // Create new gRPC service
    let grpc_svc = MdmServiceServer::new(MDMServer::new(symbol_manager));

    // Run gRPC service
    grpc_service::start(DBG, SVC_ID, cfg_manager, grpc_svc).await
}
