use mimalloc::MiMalloc;
use std::error::Error;
use std::net::SocketAddr;
use tonic::transport::{Channel, Server, Uri};

use common_config::prelude::ServiceID;
use common_service::{print_utils, shutdown_utils};
use config_manager::CfgManager;

use proto_smdb::proto::db_gateway_service_client::DbGatewayServiceClient;
use proto_smdb::proto::smdb_service_server::SmdbServiceServer;
use service::SMDBServer;
use warp::Filter;
mod service;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: ServiceID = ServiceID::SMDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup autoconfiguration.
    let svc_config = smdb_specs::smdb_service_config();

    let cfg_manager = CfgManager::new(SVC_ID, svc_config).await;

    // pull DBGW endpoint from auto config
    let (dbgw_host, dbgw_port) = cfg_manager
        .get_dbgw_host_port()
        .await
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
        .await
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

    // Build http metrics endpoint
    let routes = warp::get()
        .and(warp::path(metrics_uri.clone()))
        .and(warp::path::end())
        .and_then(get_metrics_handler);

    // Build http metrics server
    let web_addr: SocketAddr = metrics_addr.parse().expect("Failed to parse web address");

    let signal = shutdown_utils::signal_handler("metric server");
    let (_, web_server) = warp::serve(routes).bind_with_graceful_shutdown(web_addr, signal);

    // Create a handler for each server https://github.com/hyperium/tonic/discussions/740
    let grpc_handle = tokio::spawn(grpc_server);
    let http_handle = tokio::spawn(web_server);

    // Set SMDB service to online
    dbgw_client
        .set_service_online(service::get_svc_request())
        .await
        .expect("[SMDB]: Failed to set service online");

    // Start all servers jointly
    print_utils::print_start_header(&SVC_ID, &service_addr, &metrics_addr, &metrics_uri);
    match tokio::try_join!(grpc_handle, http_handle) {
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

async fn get_metrics_handler() -> Result<impl warp::Reply, warp::Rejection> {
    match autometrics::prometheus_exporter::encode_to_string() {
        Ok(metrics) => Ok(warp::reply::json(&metrics)),
        Err(_) => Err(warp::reject::not_found()),
    }
}
