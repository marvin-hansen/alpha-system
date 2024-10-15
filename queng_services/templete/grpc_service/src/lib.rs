use common_config::prelude::ServiceID;
use common_service::{print_utils, shutdown_utils};
use config_manager::CfgManager;
use smdb_client::SMDBClient;
use std::convert::Infallible;
use std::error::Error;
use tonic::body::BoxBody;
use tonic::codegen::http::{Request, Response};
use tonic::codegen::Service;
use tonic::server::NamedService;
use tonic::transport::Server;
use tonic_health::pb::health_server::{Health, HealthServer};

pub async fn start<S>(
    dbg: bool,
    svc_id: ServiceID,
    cfg_manager: CfgManager,
    grpc_svc: S,
    health_service: HealthServer<impl Health>,
) -> Result<(), Box<dyn Error>>
where
    S: Service<Request<BoxBody>, Response = Response<BoxBody>, Error = Infallible>
        + NamedService
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    let dbg_print = |msg: &str| {
        if dbg {
            println!("[{}]: {}", svc_id, msg)
        }
    };

    dbg_print("get SMDB endpoint from auto config");
    let (smdb_host, smdb_port) = cfg_manager
        .get_smdb_host_port()
        .await
        .expect("Failed to get host and port for DBGW");

    dbg_print("get dependencies from auto config");
    let dependencies = cfg_manager.get_service_dependencies();

    dbg_print("Checking whether all dependencies are online");
    let smdb_manager = SMDBClient::new(smdb_host, smdb_port).await;
    for d in dependencies {
        let available = smdb_manager
            .check_if_service_id_exists(d)
            .await
            .expect(" Failed to check if service dependency exists");

        if !available {
            panic!("Service dependency {:?} is unavailable; please start it", d);
        }
    }

    dbg_print("Configure service ip and port automatically relative to the detected context");
    let service_addr = cfg_manager
        .get_svc_socket_addr()
        .await
        .expect("Failed to get service host a       nd port");

    dbg_print("Configuring socket address for gRPC service");
    let grpc_addr = service_addr
        .parse()
        .expect("[ImsDataBinance]: Failed to parse address");

    dbg_print("Configuring gRPC server with health service and signal handler");
    let signal = shutdown_utils::signal_handler("gRPC server");
    let grpc_server = Server::builder()
        .add_service(grpc_svc)
        .add_service(health_service)
        .serve_with_shutdown(grpc_addr, signal);

    dbg_print("Configuring a new Tokio task for gRPC and HTTP server");
    // https://github.com/hyperium/tonic/discussions/740
    let grpc_handle = tokio::spawn(grpc_server);

    dbg_print("Set service online");
    smdb_manager
        .set_service_online(svc_id)
        .await
        .expect("Failed to set service online");

    // Print service start header
    print_utils::print_start_header_simple(&svc_id.name(), &service_addr);

    // Free up some memory before starting the service,
    drop(cfg_manager);
    drop(service_addr);

    // Start all servers jointly
    match tokio::try_join!(grpc_handle) {
        Ok(_) => {}
        Err(e) => {
            smdb_manager
                .set_service_offline(svc_id)
                .await
                .expect("Failed to set service offline!");
            println!(
                "Failed to start gRPC and HTTP server: {} due to error {:?}",
                svc_id, e
            );
        }
    }

    dbg_print("Set service offline");
    smdb_manager
        .set_service_offline(svc_id)
        .await
        .expect(" Failed to set service offline");

    print_utils::print_stop_header(&svc_id);

    Ok(())
}
