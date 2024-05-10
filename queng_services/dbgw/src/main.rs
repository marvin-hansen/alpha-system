use common::prelude::ServiceID;
use config_manager::CfgManager;
use ctx_manager::CtxManager;
use db_system_manager::SystemDBManager;
use dns_manager::DnsManager;
use proto_bindings::proto::db_gateway_service_server::DbGatewayServiceServer;
use service::DBGWServer;
use service_utils::{print_utils, shutdown_utils, ServiceUtil};
use std::error::Error;
use tonic::transport::Server;

mod service;
const SVC_ID: ServiceID = ServiceID::DBGW;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Pre-init setup
    let svc_util = ServiceUtil::new();
    let svc_config = svc_util.get_service_config(&SVC_ID).await;

    // Setup autoconfiguration.
    let ctx_manager = async { CtxManager::new() }.await;
    let dns_manager = async { DnsManager::new(&ctx_manager) }.await;
    let cfg_manager =
        async { CfgManager::new(SVC_ID, svc_config, &ctx_manager, &dns_manager) }.await;

    // Configure database manager
    let db_config = cfg_manager.clickhouse_config();
    let dbm = SystemDBManager::new(db_config)
        .await
        .expect("Failed to create DB Manager");

    // Configure service ip and port automatically relative to the detected context.
    let service_addr = cfg_manager
        .get_svc_socket_addr()
        .expect("DBGW: Failed to get host and port");

    // Set up socket address for gRPC and HTTP
    let grpc_addr = service_addr.parse().expect("DBGW: Failed to parse address");

    // Construct gRPC server
    let grpc_svc = DbGatewayServiceServer::new(DBGWServer::new(dbm.clone()));

    // Build gRPC server with health service and signal sigint handler
    let signal = shutdown_utils::signal_handler("gRPC server");
    let grpc_server = Server::builder()
        .add_service(grpc_svc)
        .serve_with_shutdown(grpc_addr, signal);

    // Configure http metrics endpoint ip and port automatically relative to the detected context.
    let (metrics_addr, metrics_uri) = cfg_manager
        .get_metrics_socket_addr_uri()
        .expect("DBGW: Failed to get metric host, uri, and port");

    // Create a handler for each server https://github.com/hyperium/tonic/discussions/740
    let grpc_handle = tokio::spawn(grpc_server);

    // Set DBGW service to online
    dbm.set_service_online(&SVC_ID)
        .await
        .expect("DBGW: Failed to set service online");

    // Start all servers jointly
    print_utils::print_start_header(&SVC_ID, &service_addr, &metrics_addr, &metrics_uri);
    match tokio::try_join!(grpc_handle) {
        Ok(_) => {}
        Err(e) => {
            dbm.set_service_offline(&SVC_ID)
                .await
                .expect("DBGW: Failed to set service offline!");
            println!("DBGW: Failed to start gRPC and HTTP server: {:?}", e);
        }
    }

    dbm.set_service_offline(&SVC_ID)
        .await
        .expect("DBGW: Failed to set service offline!");

    print_utils::print_stop_header(&SVC_ID);
    Ok(())
}
