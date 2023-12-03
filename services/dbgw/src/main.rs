use autometrics::prometheus_exporter;
use common::prelude::{print_utils, ServiceID};
use components::prelude::*;
use dbgw_service::service::{job::job_runner_server::*, MyJobRunner};
use std::error::Error;
use std::net::SocketAddr;
use tonic::transport::Server as TonicServer;
use warp::Filter;

mod shutdown;

const SVC_ID: ServiceID = ServiceID::DBGW;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup prometheus metrics exporter
    prometheus_exporter::init();

    // Setup autoconfiguration. All async.
    let ctx_manager = async { CtxManager::new() }.await;
    let dns_manager = async { DnsManager::new(&ctx_manager) }.await;
    let cfg_manager = async { CfgManager::new(SVC_ID, &ctx_manager) }.await;
    let svm_manager = async { EnvManager::new(&ctx_manager, &dns_manager) }.await;
    let service_manager = async { ServiceManager::new(&cfg_manager, &svm_manager) }.await;

    // Service_manager configures ip and port automatically relative to the detected context.
    let (host_ip, host_port) = service_manager
        .get_service_host_port(SVC_ID)
        .expect("DBGW: Failed to get host and port");

    // Set up two different ports for gRPC and HTTP
    let grpc_addr = format!("{}:{}", host_ip, host_port)
        .parse()
        .expect("Failed to parse address");

    let web_addr: SocketAddr = "127.0.0.1:8080"
        .parse()
        .expect("Failed to parse web address");

    // Load dbm config from config manager
    let db_config = cfg_manager.get_db_config();

    // Configure database manager
    let dbm = DBManager::new_offline(&db_config).await;

    // Construct gRPC server
    let grpc_svc = JobRunnerServer::new(MyJobRunner::default());

    // Build health service for gRPC server
    let (mut health_reporter, health_svc) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<JobRunnerServer<MyJobRunner>>()
        .await;

    // Build a gRPC sigint signal handler
    let signal = shutdown::grpc_sigint(dbm.clone());

    // Build gRPC server with health service and signal sigint handler
    let grpc_server = TonicServer::builder()
        .add_service(grpc_svc)
        .add_service(health_svc)
        .serve_with_shutdown(grpc_addr, signal);

    // Build http /metrics endpoint
    let routes = warp::get()
        .and(warp::path("metrics"))
        .map(|| prometheus_exporter::encode_http_response());

    // Build a Http sigint signal handler
    let signal = shutdown::http_sigint();

    // Build http web server
    let (_, web_server) = warp::serve(routes).bind_with_graceful_shutdown(web_addr, signal);

    // Create a handler for each server https://github.com/hyperium/tonic/discussions/740
    let grpc_handle = tokio::spawn(grpc_server);
    let grpc_web_handle = tokio::spawn(web_server);

    // Set DBGW service to online
    dbm.set_service_online(&SVC_ID)
        .await
        .expect("Failed to set service to online");

    // Print start header
    print_utils::print_start_header(&SVC_ID, grpc_addr.port());

    // Start all servers jointly
    match tokio::try_join!(grpc_handle, grpc_web_handle) {
        Ok(_) => {}
        Err(e) => {
            dbm.set_service_offline(&SVC_ID)
                .await
                .expect("Failed to set service offline");
            println!("Failed to start gRPC and HTTP server: {:?}", e);
        }
    }

    print_utils::print_stop_header(&SVC_ID);
    Ok(())
}
