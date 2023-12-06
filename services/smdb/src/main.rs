use autometrics::prometheus_exporter;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tonic::transport::Server;

use common::prelude::ServiceID::DBGW;
use common::prelude::{HostEndpoint, ServiceID};
use components::prelude::{CfgManager, CtxManager, DnsManager, EnvManager, ServiceManager};
use dbgw_client::DBGatewayClient;
use proto::binding::smdb_service_server::SmdbServiceServer;
use service_utils::{print_utils, shutdown_utils};
use warp::Filter;

use service::SMDBServer;

mod service;

const SVC_ID: ServiceID = ServiceID::SMDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup prometheus metrics exporter
    prometheus_exporter::init();

    // Setup autoconfiguration.
    let ctx_manager = async { CtxManager::new() }.await;
    let dns_manager = async { DnsManager::new(&ctx_manager) }.await;
    let cfg_manager = async { CfgManager::new(SVC_ID, &ctx_manager) }.await;
    let svm_manager = async { EnvManager::new(&ctx_manager, &dns_manager) }.await;
    let service_manager = async { ServiceManager::new(&cfg_manager, &svm_manager) }.await;

    // pull DBGW endpoint from auto config
    let (dbgw_host, dbgw_port) = service_manager
        .get_service_host_port(&DBGW)
        .expect("Failed to get host and port for DBGW");

    // Configure DBGW client
    let dbgw_endpoint = HostEndpoint::new(&dbgw_host, dbgw_port);
    let mut dbgw_client = DBGatewayClient::new(dbgw_endpoint).await;

    // Configure service ip and port automatically relative to the detected context.
    let service_addr = service_manager
        .configure_svc_socket_addr(&SVC_ID)
        .expect("DBGW: Failed to get host and port");

    // Set up socket address for gRPC and HTTP
    let grpc_addr = service_addr.parse().expect("DBGW: Failed to parse address");

    // Construct gRPC server
    let client = Arc::new(Mutex::new(dbgw_client.clone()));

    let grpc_svc = SmdbServiceServer::new(SMDBServer::new(client.clone()));

    // Build health service for gRPC server
    let (mut health_reporter, health_svc) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<SmdbServiceServer<SMDBServer>>()
        .await;

    // Build gRPC server with health service and signal sigint handler
    let signal = shutdown_utils::signal_handler("gRPC server");
    let grpc_server = Server::builder()
        .add_service(grpc_svc)
        .add_service(health_svc)
        .serve_with_shutdown(grpc_addr, signal);

    // Configure http metrics endpoint ip and port automatically relative to the detected context.
    let (metrics_addr, metrics_uri) = service_manager
        .configure_metrics_socket_addr_uri(&SVC_ID)
        .expect("DBGW: Failed to get metric host, uri, and port");

    // Http/web socket address is needed to serve metrics to prometheus
    let web_addr: SocketAddr = metrics_addr.parse().expect("DBGW: Failed to parse address");

    // Build metrics endpoint
    let routes = warp::get()
        .and(warp::path(metrics_uri.clone()))
        .map(|| prometheus_exporter::encode_http_response());

    // Build http web server for metrics with sigint handler
    let signal = shutdown_utils::signal_handler("http web server");
    let (_, web_server) = warp::serve(routes).bind_with_graceful_shutdown(web_addr, signal);

    // Create a handler for each server https://github.com/hyperium/tonic/discussions/740
    let grpc_handle = tokio::spawn(grpc_server);
    let web_handle = tokio::spawn(web_server);

    // Set SMDB service to online
    dbgw_client
        .set_service_online(SVC_ID)
        .await
        .expect("Failed to set service online");

    // Start all servers jointly
    print_utils::print_start_header(&SVC_ID, &service_addr, &metrics_addr, &metrics_uri);
    match tokio::try_join!(grpc_handle, web_handle) {
        Ok(_) => {}
        Err(e) => {
            dbgw_client
                .set_service_offline(SVC_ID)
                .await
                .expect("DBGW: Failed to set service offline!");
            println!("DBGW: Failed to start gRPC and HTTP server: {:?}", e);
        }
    }

    // Set SMDB service offline
    // dbgw_client
    //     .set_service_offline(SVC_ID)
    //     .await
    //     .expect("Failed to set service offline");

    print_utils::print_stop_header(&SVC_ID);
    Ok(())
}
