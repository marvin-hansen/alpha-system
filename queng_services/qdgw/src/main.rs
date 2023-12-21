use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use autometrics::prometheus_exporter;
use warp::Filter;

use cfg_manager::CfgManager;
use client_manager::ClientManager;
use common::prelude::ServiceID;
use common::prelude::ServiceID::SMDB;
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
use qd_manager::QDManager;
use service_utils::{print_utils, shutdown_utils};
use smdb_provider::SMDBProvider;
use svc_manager::ServiceManager;

use crate::service::Server;

mod handle_clients;
mod handle_data;
mod service;

const SVC_ID: ServiceID = ServiceID::QDGW;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //
    //Creates a new instance of the Context Manager.
    let ctx_manager = async { CtxManager::new() }.await;
    //Creates a new instance of the DNS Manager.
    let dns_manager = async { DnsManager::new(&ctx_manager) }.await;
    //Creates a new instance of the Configuration Manager.
    let cfg_manager = async { CfgManager::new(SVC_ID, &ctx_manager, &dns_manager) }.await;
    //Creates a new instance of the Service Manager.
    let service_manager = async { ServiceManager::new(&cfg_manager) }.await;

    // Creates a new instance of the QD manager.
    // Only loads 2 out of 7 local files for testing purposes.
    // Otherwise, init takes too long i.e. > 20 seconds.
    // For details, see: queng_specs/file_specs/src/files
    let qd_manager = async { QDManager::new(&cfg_manager) }.await;
    // Creates a new instance of the Client Manager.
    let client_manager = Arc::new(Mutex::new(ClientManager::new()));

    //Retrieves the host and port of the Service Manager Database (SMDB) from the auto-configuration.
    let (smdb_host, smdb_port) = service_manager
        .get_service_host_port(&SMDB)
        .expect("[QDGW]/main: Failed to get host and port for DBGW");

    //Creates a new instance of the Service Manager Database (SMDB) Provider.
    let smdb_manager = SMDBProvider::new(smdb_host, smdb_port).await;

    //Retrieves a list of all service dependencies for the current service.
    let dependencies = service_manager.get_service_dependencies();

    //Checks if the list of dependencies is empty.
    if !dependencies.is_empty() {
        //Iterates over the list of dependencies.
        for d in dependencies {
            //Checks if the Service Manager Database (SMDB) contains the specified service ID.
            let available = smdb_manager
                .check_if_service_id_exists(d)
                .await
                .expect("[QDGW]/main: Failed to check if service dependency exists");

            //Checks if the service is available.
            if !available {
                //Panics if the service is not available.
                panic!(
                    "[QDGW]/main: Service dependency {:?} is not available please start it",
                    d
                );
            }
        }
    }

    // Autoconfigures the IP address and port of the current service automatically based on the detected context.
    let _ = service_manager
        .configure_svc_socket_addr(&SVC_ID)
        .expect("[QDGW]/main: Failed to get host and port");

    // Autoconfigures the IP address and port of the HTTP metrics endpoint automatically based on the detected context.
    let (metrics_addr, metrics_uri) = service_manager
        .configure_metrics_socket_addr_uri(&SVC_ID)
        .expect("[QDGW]/main: Failed to get metric host, uri, and port");

    //Creates a SocketAddr instance from the metrics address string.
    let web_addr: SocketAddr = metrics_addr
        .parse()
        .expect("[QDGW]/main: Failed to parse metric host to address");

    //Creates a new Warp filter for the metrics endpoint.
    let routes = warp::get()
        .and(warp::path(metrics_uri.clone()))
        .map(prometheus_exporter::encode_http_response);

    //Creates a new Warp filter for the metrics endpoint with a graceful shutdown handler.
    let signal = shutdown_utils::signal_handler("http web server");
    let (_, web_server) = warp::serve(routes).bind_with_graceful_shutdown(web_addr, signal);

    // Autoconfigures message channel
    let msg_config = cfg_manager.get_message_client_config();
    let service_topic = msg_config.control_channel();

    // creates a new consumer for the topic
    let consumer = fluvio::consumer(&service_topic, 0)
        .await
        .expect("[QDGW]/main: Failed to create a message consumer");

    //Creates a new server with the specified socket
    let server = Server::new(consumer, qd_manager, client_manager);

    //Creates a new Tokio task for the HTTP web server.
    let web_handle = tokio::spawn(web_server);

    //Creates a new Tokio task for the UDP server.
    let signal = shutdown_utils::signal_handler("ZMQ server");
    let service_handle = tokio::spawn(server.run(signal));

    //Sets the current service status to online in the Service Manager Database (SMDB).
    smdb_manager
        .set_service_online(SVC_ID)
        .await
        .expect("[QDGW]/main: Failed to set service online");

    //Starts both servers concurrently.
    print_utils::print_start_header_message_service(
        &SVC_ID,
        &service_topic,
        &metrics_addr,
        &metrics_uri,
    );

    match tokio::try_join!(web_handle, service_handle) {
        Ok(_) => {}
        Err(e) => {
            //Sets the current service status to offline in the Service Manager Database (SMDB)
            // if an error occurs.
            smdb_manager
                .set_service_offline(SVC_ID)
                .await
                .expect("[QDGW]/main: Failed to set service offline!");
            println!("[QDGW]/main: Failed to start gRPC and HTTP server: {:?}", e);
        }
    }

    //Sets the current service status to offline in the Service Manager Database (SMDB).
    smdb_manager
        .set_service_offline(SVC_ID)
        .await
        .expect("[QDGW]/main: Failed to set service offline");

    //Prints the start and stop headers for the current service.
    print_utils::print_stop_header(&SVC_ID);

    Ok(())
}
