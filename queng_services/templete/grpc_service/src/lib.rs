use autometrics::prometheus_exporter;
use common_config::prelude::ServiceID;
use common_service::{print_utils, shutdown_utils};
use config_manager::CfgManager;
use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use tonic::body::BoxBody;
use tonic::codegen::http::{Request, Response};
use tonic::codegen::Service;
use tonic::server::NamedService;
use tonic::transport::Server;
use warp::Filter;

use smdb_client::SMDBClient;

pub async fn start<S>(
    dbg: bool,
    svc_id: ServiceID,
    cfg_manager: CfgManager,
    grpc_svc: S,
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
            println!("[{}]: {}", svc_id.to_string(), msg)
        }
    };

    dbg_print("Setup autometrics");
    prometheus_exporter::init();

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
        .expect("Failed to get service host and port");

    dbg_print("Configuring socket address for gRPC service");
    let grpc_addr = service_addr
        .parse()
        .expect("[ImsDataBinance]: Failed to parse address");

    dbg_print("Configuring gRPC server with health service and signal handler");
    let signal = shutdown_utils::signal_handler("gRPC server");
    let grpc_server = Server::builder()
        .add_service(grpc_svc)
        .serve_with_shutdown(grpc_addr, signal);

    dbg_print("Configuring metrics endpoint");
    let (metrics_addr, metrics_uri) = cfg_manager
        .get_metrics_socket_addr_uri()
        .expect("[ImsDataBinance]: Failed to get metric host, uri, and port");

    dbg_print("Configuring socket address for http service");
    let http_addr: SocketAddr = metrics_uri
        .parse()
        .expect("[ImsDataBinance]: Failed to parse address");

    dbg_print("Configuring health endpoint");
    let get_health_check = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(health_handler);

    dbg_print("Build metrics endpoint");
    let get_metrics = warp::get()
        .and(warp::path(metrics_uri.clone()))
        .and(warp::path::end())
        .and_then(metrics_handler);

    dbg_print("Configure http service routes");
    let routes = get_health_check.or(get_metrics);

    dbg_print("Configuring http server with health service and signal handler");
    let signal = shutdown_utils::signal_handler("http server");
    let (_, http_server) = warp::serve(routes).bind_with_graceful_shutdown(http_addr, signal);

    dbg_print("Configuring a new Tokio task for gRPC and HTTP server");
    // https://github.com/hyperium/tonic/discussions/740
    let grpc_handle = tokio::spawn(grpc_server);
    let http_handle = tokio::spawn(http_server);

    dbg_print("Set service online");
    smdb_manager
        .set_service_online(svc_id)
        .await
        .expect("Failed to set service online");

    // Print service start header
    print_utils::print_start_header(&svc_id, &service_addr, &metrics_addr, &metrics_uri);
    // Free up some memory before starting the service,
    drop(cfg_manager);
    drop(metrics_uri);
    drop(metrics_addr);
    drop(service_addr);

    // Start all servers jointly
    match tokio::try_join!(grpc_handle, http_handle) {
        Ok(_) => {}
        Err(e) => {
            smdb_manager
                .set_service_offline(svc_id)
                .await
                .expect("Failed to set service offline!");
            println!(
                "Failed to start gRPC and HTTP server: {} due to error {:?}",
                svc_id.to_string(),
                e
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

async fn health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let result = { String::from("OK") };
    Ok(warp::reply::json(&result))
}

async fn metrics_handler() -> Result<impl warp::Reply, warp::Rejection> {
    match autometrics::prometheus_exporter::encode_to_string() {
        Ok(metrics) => Ok(warp::reply::json(&metrics)),
        Err(_) => Err(warp::reject::not_found()),
    }
}
