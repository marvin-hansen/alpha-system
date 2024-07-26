use std::error::Error;

use mimalloc::MiMalloc;
use tonic::transport::{Channel, Server, Uri};

use common_config::prelude::ServiceID;
use common_service::{print_utils, shutdown_utils};
use config_manager::CfgManager;
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
use proto_bindings::proto::cmdb_service_server::CmdbServiceServer;
use proto_bindings::proto::db_gateway_service_client::DbGatewayServiceClient;
use smdb_client::SMDBClient;

use crate::service::CMDBServer;

mod service;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: ServiceID = ServiceID::CMDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup autoconfiguration.
    let svc_config = cmdb_specs::cmdb_service_config();
    let ctx_manager = async { CtxManager::new() }.await;
    let dns_manager = async { DnsManager::new(&ctx_manager) }.await;
    let cfg_manager =
        async { CfgManager::new(SVC_ID, svc_config, &ctx_manager, &dns_manager) }.await;

    // pull SMDB endpoint from auto config
    let (smdb_host, smdb_port) = cfg_manager
        .get_smdb_host_port()
        .expect("[CMDB]: Failed to get host and port for DBGW");

    let smdb_manager = SMDBClient::new(smdb_host, smdb_port).await;

    //get all dependencies
    let dependencies = cfg_manager.get_service_dependencies();

    // Check if all dependencies are online, abort of anyone is missing.
    for d in dependencies {
        let available = smdb_manager
            .check_if_service_id_exists(d)
            .await
            .expect("[CMDB]: Failed to check if service dependency exists");

        if !available {
            panic!(
                "[CMDB]: Service dependency {:?} is not available please start it",
                d
            );
        }
    }

    // pull DBGW endpoint from auto config
    let (dbgw_host, dbgw_port) = cfg_manager
        .get_dbgw_host_port()
        .expect("[CMDB]: Failed to get host and port for: DBGW");

    // Configure DBGW URI
    let s = format!("http://{}:{}", dbgw_host, dbgw_port);
    let uri = s.parse::<Uri>().unwrap();

    // Configure a channel connection to DBGW service
    let channel = Channel::builder(uri).connect().await.unwrap_or_else(|_| {
        panic!(
            "\r\n [CMDB]: Failed to connect to DBGW service on: {} \r\n  \r\n Detail: \r\n",
            s
        )
    });

    // Configure DBGW client
    let dbgw_client = DbGatewayServiceClient::new(channel);

    // Configure service ip and port automatically relative to the detected context.
    let service_addr = cfg_manager
        .get_svc_socket_addr()
        .expect("[CMDB]: Failed to get host and port");

    // Set up socket address for gRPC service
    let grpc_addr = service_addr
        .parse()
        .expect("[CMDB]: Failed to parse address");

    // Construct gRPC server
    let grpc_svc = CmdbServiceServer::new(CMDBServer::new(dbgw_client.clone()));

    // Build gRPC server with health service and signal sigint handler
    let signal = shutdown_utils::signal_handler("gRPC server");
    let grpc_server = Server::builder()
        .add_service(grpc_svc)
        .serve_with_shutdown(grpc_addr, signal);

    // Configure http metrics endpoint ip and port automatically relative to the detected context.
    let (metrics_addr, metrics_uri) = cfg_manager
        .get_metrics_socket_addr_uri()
        .expect("[CMDB]: Failed to get metric host, uri, and port");

    // Create a handler for each server https://github.com/hyperium/tonic/discussions/740
    let grpc_handle = tokio::spawn(grpc_server);

    // Set service to online
    smdb_manager
        .set_service_online(SVC_ID)
        .await
        .expect("[CMDB]: Failed to set service online");

    // Start all servers jointly
    print_utils::print_start_header(&SVC_ID, &service_addr, &metrics_addr, &metrics_uri);
    match tokio::try_join!(grpc_handle) {
        Ok(_) => {}
        Err(e) => {
            smdb_manager
                .set_service_offline(SVC_ID)
                .await
                .expect("[CMDB]: Failed to set service offline!");
            println!("[CMDB]: Failed to start gRPC and HTTP server: {:?}", e);
        }
    }

    // Set service offline
    smdb_manager
        .set_service_offline(SVC_ID)
        .await
        .expect("[CMDB]: Failed to set service offline");

    print_utils::print_stop_header(&SVC_ID);

    Ok(())
}
