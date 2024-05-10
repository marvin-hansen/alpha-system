use crate::service::SYMDBServer;
use common::prelude::ServiceID;
use config_manager::CfgManager;
use ctx_manager::CtxManager;
use db_query_manager::QueryDBManager;
use dns_manager::DnsManager;
use proto_bindings::proto::symdb_service_server::SymdbServiceServer;
use service_utils::{print_utils, shutdown_utils};
use smdb_provider::SMDBProvider;
use std::error::Error;
use std::sync::{Arc, RwLock};
use symbol_manager::SymbolManager;
use tonic::transport::Server;

mod service;

const SVC_ID: ServiceID = ServiceID::SYMDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup autoconfiguration.
    let ctx_manager = async { CtxManager::new() }.await;
    let dns_manager = async { DnsManager::new(&ctx_manager) }.await;
    let cfg_manager = async { CfgManager::new(SVC_ID, &ctx_manager, &dns_manager) }.await;

    // pull SMDB endpoint from auto config
    let (smdb_host, smdb_port) = cfg_manager
        .get_smdb_host_port()
        .expect("[SYMDB]: Failed to get host and port for DBGW");

    let smdb_manager = SMDBProvider::new(smdb_host, smdb_port).await;

    //get all dependencies
    let dependencies = cfg_manager.get_service_dependencies();

    // println!("[SYMDB]: Checking if all dependencies are online");
    for d in dependencies {
        // println!("[SYMDB]: Checking if service dependency {:?} is available", d);
        let available = smdb_manager
            .check_if_service_id_exists(d)
            .await
            .expect("[SYMDB]: Failed to check if service dependency exists");

        if !available {
            panic!(
                "[SYMDB]: Service dependency {:?} is not available please start it",
                d
            );
        }
    }

    // println!("[SYMDB]/main: Configure service ip and port automatically relative to the detected context");
    let service_addr = cfg_manager
        .get_svc_socket_addr()
        .expect("[SMDB]: Failed to get host and port");

    // println!("[SYMDB]: Configuring metrics endpoint");
    let (metrics_addr, metrics_uri) = cfg_manager
        .get_metrics_socket_addr_uri()
        .expect("[SYMDB]: Failed to get metric host, uri, and port");

    // println!("[SYMDB]: Get the symbol table for the default exchange.");
    let default_exchange = cfg_manager.default_exchange();
    let exchange_symbol_table = cfg_manager
        .get_symbol_table(default_exchange)
        .expect("[SYMDB]/main: Failed to get symbol table for default exchange.");
    // println!("[SYMDB]: Symbol table for the default exchange: {}",exchange_symbol_table);

    // println!("[SYMDB]: Create a new QueryDBManager instance.");
    let db_config = cfg_manager.clickhouse_config().to_owned();
    let mut q_manager = QueryDBManager::new(db_config)
        .await
        .expect("[SYMDB]/main: Failed to create QueryDBManager instance.");

    // println!("[SYMDB]: Get all symbols for the default exchange.");
    let symbols = q_manager
        .get_all_symbols_with_ids(&exchange_symbol_table)
        .await
        .expect("[SYMDB]/main: Failed to get all symbols for SymbolManager.");

    let exchanges = cfg_manager.exchanges_id_names().to_owned();

    // println!("[SYMDB]: Create a new SymbolManager instance.");
    let symbol_manager = async {
        Arc::new(RwLock::new(SymbolManager::new(symbols, exchanges).expect(
            "[SYMDB]/main: Failed to create SymbolManager instance.",
        )))
    }
    .await;

    // Set up socket address for gRPC service
    let grpc_addr = service_addr
        .parse()
        .expect("[SMDB]: Failed to parse address");

    // Create new gRPC service
    let grpc_svc = SymdbServiceServer::new(SYMDBServer::new(symbol_manager));

    // Build gRPC server with health service and signal sigint handler
    let signal = shutdown_utils::signal_handler("gRPC server");
    let grpc_server = Server::builder()
        .add_service(grpc_svc)
        .serve_with_shutdown(grpc_addr, signal);

    //Creates a new Tokio task for each server.
    // https://github.com/hyperium/tonic/discussions/740
    let grpc_handle = tokio::spawn(grpc_server);

    // Print service start header
    print_utils::print_start_header(&SVC_ID, &service_addr, &metrics_addr, &metrics_uri);

    // Free up some memory before starting the service,
    drop(cfg_manager);
    drop(metrics_uri);
    drop(metrics_addr);
    drop(q_manager);
    drop(service_addr);

    // Set service to online
    smdb_manager
        .set_service_online(SVC_ID)
        .await
        .expect("[SYMDB]: Failed to set service online");

    // Start all servers jointly
    match tokio::try_join!(grpc_handle) {
        Ok(_) => {}
        Err(e) => {
            smdb_manager
                .set_service_offline(SVC_ID)
                .await
                .expect("[SYMDB]: Failed to set service offline!");
            println!("[SYMDB]: Failed to start gRPC and HTTP server: {:?}", e);
        }
    }

    // Set service offline
    smdb_manager
        .set_service_offline(SVC_ID)
        .await
        .expect("[SYMDB]: Failed to set service offline");

    print_utils::print_stop_header(&SVC_ID);

    Ok(())
}
