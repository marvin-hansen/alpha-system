use config_manager::CfgManager;
use std::error::Error;
use tonic::transport::{Channel, Server, Uri};

use common::prelude::ServiceID;
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
use proto_bindings::proto::db_gateway_service_client::DbGatewayServiceClient;
use proto_bindings::proto::smdb_service_server::SmdbServiceServer;
use service::SMDBServer;
use service_utils::{print_utils, shutdown_utils};

mod service;

const SVC_ID: ServiceID = ServiceID::SMDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup autoconfiguration.
    let ctx_manager = async { CtxManager::new() }.await;
    let dns_manager = async { DnsManager::new(&ctx_manager) }.await;
    let cfg_manager = async { CfgManager::new(SVC_ID, &ctx_manager, &dns_manager) }.await;

    // pull DBGW endpoint from auto config
    let (dbgw_host, dbgw_port) = cfg_manager
        .get_dbgw_host_port()
        .expect("[SMDB]: Failed to get host and port for DBGW");

    // Configure DBGW URI
    let s = format!("http://{}:{}", dbgw_host, dbgw_port);
    let uri = s.parse::<Uri>().unwrap();

    // Configure a channel connection to DBGW service
    let channel = Channel::builder(uri).connect().await.unwrap_or_else(|_| {
        panic!(
            "\r\n [SMDB]: Failed to connect to DBGW service on: {} \r\n  \r\n Detail: \r\n",
            s
        )
    });

    // Configure DBGW client
    let mut dbgw_client = DbGatewayServiceClient::new(channel);

    // Configure service ip and port automatically relative to the detected context.
    let service_addr = cfg_manager
        .get_svc_socket_addr()
        .expect("[SMDB]: Failed to get host and port");

    // Set up socket address for gRPC service
    let grpc_addr = service_addr
        .parse()
        .expect("[SMDB]: Failed to parse address");

    // Construct gRPC server
    let grpc_svc = SmdbServiceServer::new(SMDBServer::new(dbgw_client.clone()));

    // Build gRPC server with health service and signal sigint handler
    let signal = shutdown_utils::signal_handler("gRPC server");
    let grpc_server = Server::builder()
        .add_service(grpc_svc)
        .serve_with_shutdown(grpc_addr, signal);

    // Configure http metrics endpoint ip and port automatically relative to the detected context.
    let (metrics_addr, metrics_uri) = cfg_manager
        .get_metrics_socket_addr_uri()
        .expect("[SMDB]: Failed to get metric host, uri, and port");

    // Create a handler for each server https://github.com/hyperium/tonic/discussions/740
    let grpc_handle = tokio::spawn(grpc_server);

    // Set SMDB service to online
    dbgw_client
        .set_service_online(service::get_svc_request())
        .await
        .expect("[SMDB]: Failed to set service online");

    // Start all servers jointly
    print_utils::print_start_header(&SVC_ID, &service_addr, &metrics_addr, &metrics_uri);
    match tokio::try_join!(grpc_handle) {
        Ok(_) => {}
        Err(e) => {
            dbgw_client
                .set_service_offline(service::get_svc_request())
                .await
                .expect("[SMDB]: Failed to set service offline!");
            println!("[SMDB]: Failed to start gRPC and HTTP server: {:?}", e);
        }
    }

    // Set SMDB service offline
    dbgw_client
        .set_service_offline(service::get_svc_request())
        .await
        .expect("[SMDB]: Failed to set service offline");

    print_utils::print_stop_header(&SVC_ID);
    Ok(())
}
