use std::error::Error;

use tonic::transport::Server;

use common_config::prelude::{ServiceConfig, ServiceID};
use common_service::{print_utils, shutdown_utils};
use config_manager::CfgManager;
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
use proto_bindings::proto::ims_data_service_server::{ImsDataService, ImsDataServiceServer};
use smdb_client::SMDBClient;

pub async fn run(
    svc_id: ServiceID,
    svc_config: ServiceConfig,
    grpc_svc: ImsDataServiceServer<impl ImsDataService>,
) -> Result<(), Box<dyn Error>> {
    //
    //Creates a new instance of the Context Manager.
    let ctx_manager = async { CtxManager::new() }.await;
    //Creates a new instance of the DNS Manager.
    let dns_manager = async { DnsManager::new(&ctx_manager) }.await;
    //Creates a new instance of the Configuration Manager.
    let cfg_manager =
        async { CfgManager::new(svc_id, svc_config, &ctx_manager, &dns_manager) }.await;

    // pull SMDB endpoint from auto config
    let (smdb_host, smdb_port) = cfg_manager
        .get_smdb_host_port()
        .expect("[ImsDataBinance]: Failed to get host and port for DBGW");

    let smdb_manager = SMDBClient::new(smdb_host, smdb_port).await;

    //get all dependencies
    let dependencies = cfg_manager.get_service_dependencies();

    // println!("[ImsDataBinance]: Checking if all dependencies are online");
    for d in dependencies {
        // println!("[ImsDataBinance]: Checking if service dependency {:?} is available", d);
        let available = smdb_manager
            .check_if_service_id_exists(d)
            .await
            .expect("[ImsDataBinance]: Failed to check if service dependency exists");

        if !available {
            panic!(
                "[ImsDataBinance]: Service dependency {:?} is not available please start it",
                d
            );
        }
    }

    // println!("[ImsDataBinance]/main: Configure service ip and port automatically relative to the detected context");
    let service_addr = cfg_manager
        .get_svc_socket_addr()
        .expect("[ImsDataBinance]: Failed to get host and port");

    // println!("[ImsDataBinance]: Configuring metrics endpoint");
    let (metrics_addr, metrics_uri) = cfg_manager
        .get_metrics_socket_addr_uri()
        .expect("[ImsDataBinance]: Failed to get metric host, uri, and port");

    // Set up socket address for gRPC service
    let grpc_addr = service_addr
        .parse()
        .expect("[ImsDataBinance]: Failed to parse address");

    // Build gRPC server with health service and signal sigint handler
    let signal = shutdown_utils::signal_handler("gRPC server");
    let grpc_server = Server::builder()
        .add_service(grpc_svc)
        .serve_with_shutdown(grpc_addr, signal);

    //Creates a new Tokio task for each server.
    // https://github.com/hyperium/tonic/discussions/740
    let grpc_handle = tokio::spawn(grpc_server);

    // Print service start header
    print_utils::print_start_header(&svc_id, &service_addr, &metrics_addr, &metrics_uri);

    // Free up some memory before starting the service,
    drop(cfg_manager);
    drop(metrics_uri);
    drop(metrics_addr);

    // Set service to online
    smdb_manager
        .set_service_online(svc_id)
        .await
        .expect("[ImsDataBinance]: Failed to set service online");

    // Start all servers jointly
    match tokio::try_join!(grpc_handle) {
        Ok(_) => {}
        Err(e) => {
            smdb_manager
                .set_service_offline(svc_id)
                .await
                .expect("[ImsDataBinance]: Failed to set service offline!");
            println!(
                "[ImsDataBinance]: Failed to start gRPC and HTTP server: {:?}",
                e
            );
        }
    }

    // Set service offline
    smdb_manager
        .set_service_offline(svc_id)
        .await
        .expect("[ImsDataBinance]: Failed to set service offline");

    print_utils::print_stop_header(&svc_id);

    Ok(())
}
