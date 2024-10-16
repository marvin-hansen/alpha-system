use mimalloc::MiMalloc;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::Server;
use warp::Filter;

use crate::service_cmdb::CMDBServer;
use crate::service_smdb::SMDBServer;
use common_config::prelude::ServiceID;
use common_service::{print_utils, shutdown_utils};
use config_manager::CfgManager;
use pg_cmdb_manager::PostgresCMDBManager;
use pg_smdb_manager::PostgresSMDBManager;
use postgres_config_manager::PostgresConfigManager;
use proto_dbgw::proto::db_gateway_cmdb_service_server::DbGatewayCmdbServiceServer;
use proto_dbgw::proto::db_gateway_smdb_service_server::DbGatewaySmdbServiceServer;

mod service_cmdb;
mod service_smdb;

const DBG: bool = true;
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
    let cfg_manager = CfgManager::build(DBG, SVC_ID, svc_config);
    let pg_cfg_manager = PostgresConfigManager::new(&cfg_manager.env_type());

    dbg_print(&format!("Detected context: {}", cfg_manager.env_type()));

    dbg_print("Configure service ip and port for the detected context");
    let service_addr = cfg_manager
        .get_svc_socket_addr()
        .await
        .expect("DBGW: Failed to get host and port");
    dbg_print(&service_addr);

    dbg_print("Set up socket address for gRPC and HTTP");
    let grpc_addr = service_addr.parse().expect("DBGW: Failed to parse address");

    dbg_print("Configure postgres database manager");
    let pg_config = pg_cfg_manager.postgres_db_config();

    let dbm_smdb = PostgresSMDBManager::new(&pg_config.pg_connection_url())
        .await
        .expect("Failed to create DB Manager");

    let dbm_cmdb = PostgresCMDBManager::new(&pg_config.pg_connection_url())
        .await
        .expect("Failed to create DB Manager");

    let arc_smdb_dbm = Arc::new(RwLock::new(dbm_smdb));
    let arc_dbm_cmdb = Arc::new(RwLock::new(dbm_cmdb));

    dbg_print("Construct gRPC health_service");
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<DbGatewaySmdbServiceServer<SMDBServer>>()
        .await;

    dbg_print("Construct gRPC server");
    let grpc_cmdb = DbGatewayCmdbServiceServer::new(CMDBServer::new(arc_dbm_cmdb));
    let grpc_smdb = DbGatewaySmdbServiceServer::new(SMDBServer::new(arc_smdb_dbm.clone()));

    let signal = shutdown_utils::signal_handler("gRPC server");
    let grpc_server = Server::builder()
        .add_service(grpc_cmdb)
        .add_service(grpc_smdb)
        .add_service(health_service)
        .serve_with_shutdown(grpc_addr, signal);

    dbg_print("Create an async handler for gRPC server");
    let grpc_handle = tokio::spawn(grpc_server);

    dbg_print("Configuring metrics endpoint");
    let (metrics_addr, metrics_uri) = cfg_manager
        .get_metrics_socket_addr_uri()
        .expect("DBGW: Failed to get metric host, uri, and port");
    dbg_print(&metrics_addr);

    dbg_print("Configuring health endpoint");
    let health_uri = "health";
    let get_health_check = warp::get()
        .and(warp::path(health_uri))
        .and(warp::path::end())
        .and_then(health_handler);

    dbg_print("Configure http service routes");
    let routes = get_health_check;

    dbg_print("Configuring socket address for http service");
    let http_addr: SocketAddr = metrics_addr
        .parse()
        .expect("[DBGW/main]: Failed to parse address");

    let signal = shutdown_utils::signal_handler("http server");
    let (_, http_server) = warp::serve(routes).bind_with_graceful_shutdown(http_addr, signal);

    dbg_print("Create an async handler for http server");
    let http_handle = tokio::spawn(http_server);

    {
        dbg_print("Set DBGW service to online");
        let dbm = arc_smdb_dbm.write().await;
        dbm.set_service_online(&SVC_ID)
            .await
            .expect("DBGW: Failed to set service online");
    }

    print_utils::print_start_header(
        &SVC_ID,
        &service_addr,
        &metrics_addr,
        &metrics_uri,
        health_uri,
    );
    match tokio::try_join!(grpc_handle, http_handle) {
        Ok(_) => {}
        Err(e) => {
            // Set DBGW service to offline in case of an error during gRPC start up
            let dbm = arc_smdb_dbm.write().await;
            dbm.set_service_offline(&SVC_ID)
                .await
                .expect("DBGW: Failed to set service online");
            println!("DBGW: Failed to start gRPC and HTTP server: {:?}", e);
        }
    }

    {
        dbg_print("Set DBGW service to OFFLINE");
        let dbm = arc_smdb_dbm.write().await;
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
