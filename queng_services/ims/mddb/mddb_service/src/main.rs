mod service;

use std::error::Error;

use crate::service::MDDBServer;
use common_config::prelude::ServiceID;
use config_manager::CfgManager;
use mimalloc::MiMalloc;
use proto_mddb::proto::db_gateway_mddb_service_client::DbGatewayMddbServiceClient;
use proto_mddb::proto::mddb_service_server::MddbServiceServer;
use tonic::transport::{Channel, Uri};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SVC_ID: ServiceID = ServiceID::MDDB;
const DBG: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dbg_print("Setup autoconfiguration");
    let svc_config = mddb_specs::mddb_service_config();
    let cfg_manager = CfgManager::new(SVC_ID, svc_config).await;
    dbg_print(&format!("Detected context: {}", cfg_manager.env_type()));

    dbg_print("Pull DBGW endpoint from auto config");
    let (dbgw_host, dbgw_port) = cfg_manager
        .get_dbgw_host_port()
        .await
        .expect("[MDDB]: Failed to get host and port for DBGW");

    dbg_print("Configure DBGW URI");
    let s = format!("http://{}:{}", dbgw_host, dbgw_port);
    let uri = s.parse::<Uri>().unwrap();
    dbg_print(&uri.to_string());

    dbg_print("Connect to DBGW service");
    let channel = Channel::builder(uri).connect().await.unwrap_or_else(|_| {
        panic!(
            "\r\n [MDDB]: Failed to connect to DBGW service on: {} \r\n  \r\n Detail: \r\n",
            s
        )
    });

    dbg_print("Configure gRPC server");
    let dbgw_client = DbGatewayMddbServiceClient::new(channel);
    let grpc_svc = MddbServiceServer::new(MDDBServer::new(dbgw_client));
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<MddbServiceServer<MDDBServer>>()
        .await;

    dbg_print("Start CMDB gRPC server");
    grpc_service::start(DBG, SVC_ID, cfg_manager, grpc_svc, health_service).await
}

fn dbg_print(msg: &str) {
    if DBG {
        println!("[MDDB/main]: {}", msg)
    }
}
