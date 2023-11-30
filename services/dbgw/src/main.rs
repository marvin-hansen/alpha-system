use std::error::Error;
use tokio::{
    signal::unix::{signal, SignalKind}, spawn,
    sync::oneshot::{self, Receiver, Sender},
};
use tonic::transport::Server;
use common::prelude::{print_utils, ServiceID};
use components::prelude::*;
use dbgw_service::service::{
    job::job_runner_server::*,
    MyJobRunner
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup autoconfiguration
    let svc_id = ServiceID::DBGW;
    let ctx_manager = CtxManager::new();
    let dns_manager = DnsManager::new(&ctx_manager);
    let cfg_manager = CfgManager::new(svc_id, &ctx_manager);
    let svm_manager = EnvManager::new(&ctx_manager, &dns_manager);
    let service_manager = ServiceManager::new(&cfg_manager, &svm_manager);

    // service_manager configures ip and port automatically relative to the detected context.
    let (host_ip, port) = service_manager
        .get_service_host_port(svc_id)
        .expect("DBGW: Failed to get host and port");

    // Parse ip and port into a socket address
    let addr =  format!("{}:{}", host_ip, port)
        .parse()
        .expect("Failed to parse address");

    // Load dbm config from config manager relative to the detected context.
    let db_config = cfg_manager.get_db_config();
    // Configure database manager
    let dbm = DBManager::new_offline(&db_config).await;

    // Set DBGW service to online
    dbm.set_service_online(&svc_id)
        .await
        .expect("Failed to set service to online");

    // Construct gRPC server
    let svc = JobRunnerServer::new(MyJobRunner::default());

    // Construct health service for gRPC server
    let (mut health_reporter, health_svc) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<JobRunnerServer<MyJobRunner>>().await;

    // Construct sigint signal handler
    let (signal_tx, signal_rx) = signal_channel();
    spawn(handle_sigterm(signal_tx));

    // Build gRPC server with health service and signal handler
    let server = Server::builder()
        .add_service(svc)
        .add_service(health_svc)
        .serve_with_shutdown(addr, async {
            signal_rx.await.ok();
        });

    // Start server
    print_utils::print_start_header(&svc_id, port);
    server
        .await
        .expect("Failed to start server");

    Ok(())
}

fn signal_channel() -> (Sender<()>, Receiver<()>) {
    oneshot::channel()
}

async fn handle_sigterm(tx: Sender<()>) {
    let _ = signal(SignalKind::terminate())
        .expect("failed to install signal handler")
        .recv()
        .await;

    println!("SIGTERM received: shutting down");
    let _ = tx.send(());
}
