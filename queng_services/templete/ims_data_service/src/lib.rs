use crate::service::Server;
use common_exchange::ExchangeID;
use common_service::{print_utils, shutdown_utils};
use config_manager::CfgManager;
use smdb_client::SMDBClient;
use tokio::time::Instant;

mod handle;
mod run;
mod service;
mod utils;

pub async fn start(
    dbg: bool,
    cfg_manager: CfgManager,
    exchange_id: ExchangeID,
) -> Result<(), Box<dyn std::error::Error>> {
    let svc_name = &format!("IMS {} Data Service", exchange_id);
    let dbg_print = |msg: &str| {
        if dbg {
            println!("[{}]: {}", svc_name, msg)
        }
    };
    let start = Instant::now();

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

    dbg_print("Configuring server and signal handler");
    //Creates a new server
    let server = Server::new().await;
    let signal = shutdown_utils::signal_handler("gRPC server");
    let service_handle = tokio::spawn(server.run(signal));

    dbg_print("Set integration online");

    // Free up some memory before starting the service,
    drop(cfg_manager);

    // Print service start header
    print_utils::print_duration("Starting service took:", &start.elapsed());
    print_utils::print_start_header_simple(svc_name, &service_addr);

    //Start server.
    match tokio::try_join!(service_handle) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "[{svc_name}]/main: Failed to start Message service: {:?}",
                e
            );
        }
    }
    //
    dbg_print("Set integration offline");

    Ok(())
}
