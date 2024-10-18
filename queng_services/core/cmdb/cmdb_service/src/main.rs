use common_config::prelude::ServiceID;
use config_manager::CfgManager;
use mimalloc::MiMalloc;
use std::error::Error;
use tonic::transport::{Channel, Uri};

use crate::service::CMDBServer;
use proto_cmdb::proto::cmdb_service_server::CmdbServiceServer;
use proto_cmdb::proto::db_gateway_cmdb_service_client::DbGatewayCmdbServiceClient;

mod service;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: ServiceID = ServiceID::CMDB;
const DBG: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dbg_print("Setup autoconfiguration");
    let svc_config = cmdb_specs::cmdb_service_config();
    let cfg_manager = CfgManager::new(SVC_ID, svc_config).await;
    dbg_print(&format!("Detected context: {}", cfg_manager.env_type()));

    dbg_print("Pull DBGW endpoint from auto config");
    let (dbgw_host, dbgw_port) = cfg_manager
        .get_dbgw_host_port()
        .await
        .expect("[CMDB]: Failed to get host and port for DBGW");

    dbg_print("Configure DBGW URI");
    let s = format!("http://{}:{}", dbgw_host, dbgw_port);
    let uri = s.parse::<Uri>().unwrap();
    dbg_print(&uri.to_string());

    dbg_print("Connect to DBGW service");
    let channel = Channel::builder(uri).connect().await.unwrap_or_else(|_| {
        panic!(
            "\r\n [CMDB]: Failed to connect to DBGW service on: {} \r\n  \r\n Detail: \r\n",
            s
        )
    });

    dbg_print("Configure gRPC server");
    let dbgw_client = DbGatewayCmdbServiceClient::new(channel);
    let grpc_svc = CmdbServiceServer::new(CMDBServer::new(dbgw_client));
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<CmdbServiceServer<CMDBServer>>()
        .await;

    dbg_print("Start CMDB gRPC server");
    grpc_service::start(DBG, SVC_ID, cfg_manager, grpc_svc, health_service).await
}

fn dbg_print(msg: &str) {
    if DBG {
        println!("[CMDB/main]: {}", msg)
    }
}
