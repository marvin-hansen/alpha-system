use crate::service::Server;
use autometrics::prometheus_exporter;
use cfg_manager::CfgManager;
use common::prelude::ServiceID;
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
use service_utils::{print_utils, shutdown_utils};
use smdb_provider::SMDBProvider;
use std::error::Error;
use std::net::SocketAddr;
use svc_manager::ServiceManager;
use warp::Filter;

mod service;

const SVC_ID: ServiceID = ServiceID::VEX;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup autoconfiguration.
    let ctx_manager = async { CtxManager::new() }.await;
    let dns_manager = async { DnsManager::new(&ctx_manager) }.await;
    let cfg_manager = async { CfgManager::new(SVC_ID, &ctx_manager, &dns_manager) }.await;
    let service_manager = async { ServiceManager::new(&cfg_manager) }.await;

    // pull SMDB endpoint from auto config
    let (smdb_host, smdb_port) = service_manager
        .get_service_host_port(&SVC_ID)
        .expect("[CMDB]: Failed to get host and port for DBGW");

    let smdb_manager = SMDBProvider::new(smdb_host, smdb_port).await;

    //get all dependencies
    let dependencies = service_manager.get_service_dependencies();

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

    // Configure service ip and port automatically relative to the detected context.
    let service_addr = service_manager
        .configure_svc_socket_addr(&SVC_ID)
        .expect("[CMDB]: Failed to get host and port");

    // Configure http metrics endpoint ip and port automatically relative to the detected context.
    let (metrics_addr, metrics_uri) = service_manager
        .configure_metrics_socket_addr_uri(&SVC_ID)
        .expect("[CMDB]: Failed to get metric host, uri, and port");

    // Http/web socket address is needed to serve metrics to prometheus
    let web_addr: SocketAddr = metrics_addr
        .parse()
        .expect("[CMDB]: Failed to parse metric host to address");

    // Build metrics endpoint
    let routes = warp::get()
        .and(warp::path(metrics_uri.clone()))
        .map(prometheus_exporter::encode_http_response);

    // Build http web server for metrics with sigint handler
    let signal = shutdown_utils::signal_handler("http web server");
    let (_, web_server) = warp::serve(routes).bind_with_graceful_shutdown(web_addr, signal);

    // Create server
    let server = Server::new();
    let signal = shutdown_utils::signal_handler("ZMQ server");

    // Create task handles to start servers
    let service_handle = tokio::spawn(server.run(signal));
    let web_handle = tokio::spawn(web_server);

    // Set service to online
    smdb_manager
        .set_service_online(SVC_ID)
        .await
        .expect("[CMDB]: Failed to set service online");

    // Start all servers jointly
    print_utils::print_start_header(&SVC_ID, &service_addr, &metrics_addr, &metrics_uri);
    match tokio::try_join!(web_handle, service_handle) {
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
