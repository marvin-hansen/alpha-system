use std::error::Error;
use std::sync::Arc;

use mimalloc::MiMalloc;
use tokio::sync::RwLock;
use tonic::transport::Server;
use warp::Filter;

use common_config::prelude::ServiceID;
use common_service::{print_utils, shutdown_utils};
use config_manager::CfgManager;
use ctx_manager::CtxManager;
use db_postgres_manager::PostgresDBManager;
use dns_manager::DnsManager;
use proto_bindings::proto::db_gateway_service_server::DbGatewayServiceServer;
use service::DBGWServer;

mod service;
const DBG: bool = false;
const SVC_ID: ServiceID = ServiceID::DBGW;

// Overwrites the default memory allocator.
// This fixes a performance issue due to threat contention in the MUSL memory allocator.
// https://www.linkedin.com/pulse/testing-alternative-c-memory-allocators-pt-2-musl-mystery-gomes
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dbg_print("Setup autoconfiguration");
    let svc_config = dbgw_specs::dbgw_service_config();

    let ctx_manager = async { CtxManager::new() }.await;
    let dns_manager = async { DnsManager::new(&ctx_manager) }.await;
    let cfg_manager =
        async { CfgManager::new(SVC_ID, svc_config, &ctx_manager, &dns_manager) }.await;
    dbg_print(&format!("Detected context: {}", ctx_manager.env_type()));

    dbg_print("Configure service ip and port for the detected context");
    let service_addr = cfg_manager
        .get_svc_socket_addr()
        .expect("DBGW: Failed to get host and port");

    dbg_print("Set up socket address for gRPC and HTTP");
    let grpc_addr = service_addr.parse().expect("DBGW: Failed to parse address");

    dbg_print("Configure postgres database manager");
    let pg_config = cfg_manager.postgres_db_config();
    let dbm = PostgresDBManager::new(pg_config)
        .await
        .expect("Failed to create DB Manager");

    let arc_dbm = Arc::new(RwLock::new(dbm));

    dbg_print("Construct gRPC health_service");
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<DbGatewayServiceServer<DBGWServer>>()
        .await;

    dbg_print("Construct gRPC server");
    let grpc_svc = DbGatewayServiceServer::new(DBGWServer::new(arc_dbm.clone()));
    let signal = shutdown_utils::signal_handler("gRPC server");
    let grpc_server = Server::builder()
        .add_service(grpc_svc)
        .add_service(health_service)
        .serve_with_shutdown(grpc_addr, signal);

    dbg_print("Create an async handler for gRPC server");
    let grpc_handle = tokio::spawn(grpc_server);
    {
        dbg_print("Set DBGW service to online");
        let dbm = arc_dbm.write().await;
        dbm.set_service_online(&SVC_ID)
            .await
            .expect("DBGW: Failed to set service online");
    }

    // Configure http metrics endpoint ip and port automatically relative to the detected context.
    let (metrics_addr, metrics_uri) = cfg_manager
        .get_metrics_socket_addr_uri()
        .expect("DBGW: Failed to get metric host, uri, and port");

    let health_check = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(health_handler);

    let signal = shutdown_utils::signal_handler("http server");

    let (_, http_server) =
        warp::serve(health_check).bind_with_graceful_shutdown(([127, 0, 0, 1], 8080), signal);

    dbg_print("Create an async handler for http server");
    let http_handle = tokio::spawn(http_server);

    print_utils::print_start_header(&SVC_ID, &service_addr, &metrics_addr, &metrics_uri);
    match tokio::try_join!(grpc_handle, http_handle) {
        Ok(_) => {}
        Err(e) => {
            // Set DBGW service to offline in case of an error during gRPC start up
            let dbm = arc_dbm.write().await;
            dbm.set_service_offline(&SVC_ID)
                .await
                .expect("DBGW: Failed to set service online");
            println!("DBGW: Failed to start gRPC and HTTP server: {:?}", e);
        }
    }

    {
        dbg_print("Set DBGW service to OFFLINE");
        let dbm = arc_dbm.write().await;
        dbm.set_service_offline(&SVC_ID)
            .await
            .expect("DBGW: Failed to set service online");
    }

    print_utils::print_stop_header(&SVC_ID);
    Ok(())
}

pub(crate) async fn health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let result = { String::from("OK") };
    Ok(warp::reply::json(&result))
}

fn dbg_print(msg: &str) {
    if DBG {
        println!("[DBGW/main]: {}", msg)
    }
}
