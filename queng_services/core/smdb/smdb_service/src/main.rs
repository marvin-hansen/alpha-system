use mimalloc::MiMalloc;
use std::error::Error;
use tokio::time::Instant;
use tonic::transport::{Channel, Server, Uri};

use common_config::ServiceID;
use common_service::{print_utils, shutdown_utils};
use config_manager::CfgManager;
use proto_smdb::proto::db_gateway_smdb_service_client::DbGatewaySmdbServiceClient;

use proto_smdb::proto::smdb_service_server::SmdbServiceServer;
use service::SMDBServer;
mod service;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
const DBG: bool = false;
const SVC_ID: ServiceID = ServiceID::SMDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    dbg_print("Setup autoconfiguration");
    let svc_config = smdb_specs::smdb_service_config();

    dbg_print("Setup ConfigManager");
    let cfg_manager = CfgManager::build(DBG, SVC_ID, svc_config);

    dbg_print(&format!("Detected context: {}", cfg_manager.env_type()));

    dbg_print("Pull DBGW endpoint from auto config");
    let (dbgw_host, dbgw_port) = cfg_manager
        .get_dbgw_host_port()
        .await
        .expect("[SMDB]: Failed to get host and port for DBGW");

    dbg_print("Configure DBGW URI");
    let s = format!("http://{dbgw_host}:{dbgw_port}");
    let uri = s.parse::<Uri>().unwrap();
    dbg_print(&uri.to_string());

    dbg_print("Connect to DBGW service");
    let channel = Channel::builder(uri).connect().await.unwrap_or_else(|_| {
        panic!("\r\n [SMDB]: Failed to connect to DBGW service on: {s} \r\n  \r\n Detail: \r\n")
    });

    dbg_print("Configure DBGW client");
    let mut dbgw_client = DbGatewaySmdbServiceClient::new(channel);

    dbg_print("Configure service ip and port for the detected context");
    let service_addr = cfg_manager
        .get_svc_socket_addr()
        .await
        .expect("[SMDB]: Failed to get host and port");

    dbg_print("Set up socket address for gRPC");
    let grpc_addr = service_addr
        .parse()
        .expect("[SMDB]: Failed to parse address");

    dbg_print("Set up health check url");
    let health_uri = cfg_manager
        .get_health_check_url(&SVC_ID)
        .await
        .expect("[SMDB]: Failed to get health check url");

    dbg_print("Construct gRPC health_service");
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<SmdbServiceServer<SMDBServer>>()
        .await;

    dbg_print("Construct gRPC server");
    let grpc_svc = SmdbServiceServer::new(SMDBServer::new(dbgw_client.clone()));
    let signal = shutdown_utils::signal_handler("gRPC server");
    let grpc_server = Server::builder()
        .add_service(grpc_svc)
        .add_service(health_service)
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
    print_utils::print_duration("Starting SMDB took:", &start.elapsed());
    print_utils::print_start_header(
        &SVC_ID,
        &service_addr,
        &metrics_addr,
        &metrics_uri,
        &health_uri,
    );
    match tokio::try_join!(grpc_handle) {
        Ok(_) => {}
        Err(e) => {
            dbgw_client
                .set_service_offline(service::get_svc_request())
                .await
                .expect("[SMDB]: Failed to set service offline!");
            println!("[SMDB]: Failed to start gRPC server: {e:?}");
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

fn dbg_print(msg: &str) {
    if DBG {
        println!("[SMDB/main]: {msg}");
    }
}
