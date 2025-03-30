/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

mod service;

use crate::service::IMDBServer;
use common_config::ServiceID;
use config_manager::CfgManager;
use mimalloc::MiMalloc;
use proto_imdb::proto::db_gateway_imdb_service_client::DbGatewayImdbServiceClient;
use proto_imdb::proto::imdb_service_server::ImdbServiceServer;
use std::error::Error;
use tonic::transport::{Channel, Uri};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: ServiceID = ServiceID::IMDB;
const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dbg_print("Setup autoconfiguration");
    let svc_config = imdb_specs::imdb_service_config();
    let cfg_manager = CfgManager::new(SVC_ID, svc_config).await;
    dbg_print(&format!("Detected context: {}", cfg_manager.env_type()));

    dbg_print("Pull DBGW endpoint from auto config");
    let (dbgw_host, dbgw_port) = cfg_manager
        .get_dbgw_host_port()
        .await
        .expect("[MDDB]: Failed to get host and port for DBGW");

    dbg_print("Configure DBGW URI");
    let s = format!("http://{dbgw_host}:{dbgw_port}");
    let uri = s.parse::<Uri>().unwrap();
    dbg_print(&uri.to_string());

    dbg_print("Connect to DBGW service");
    let channel = Channel::builder(uri).connect().await.unwrap_or_else(|_| {
        panic!("\r\n [MDDB]: Failed to connect to DBGW service on: {s} \r\n  \r\n Detail: \r\n")
    });

    let dbgw_client = DbGatewayImdbServiceClient::new(channel);

    dbg_print("Configure gRPC server");
    let grpc_svc = ImdbServiceServer::new(IMDBServer::new(dbgw_client));
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<ImdbServiceServer<IMDBServer>>()
        .await;
    // dbg_print("Start CMDB gRPC server");
    grpc_service::start(DBG, SVC_ID, cfg_manager, grpc_svc, health_service).await
}

fn dbg_print(msg: &str) {
    if DBG {
        println!("[IMDB/main]: {msg}");
    }
}
