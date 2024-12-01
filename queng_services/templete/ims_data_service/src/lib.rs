use crate::service::Service;
use common_exchange::ExchangeID;
use common_iggy::IggyConfig;
use common_ims::IntegrationConfig;
use common_service::print_utils;
use config_manager::CfgManager;
use iggy::client::{Client, UserClient};
use smdb_client::SMDBClient;
use tokio::time::Instant;

mod auth;
mod handle;
mod run;
mod service;
mod shutdown;
mod utils;

pub async fn start(
    dbg: bool,
    exchange_id: ExchangeID,
    integration_config: &IntegrationConfig,
    iggy_config: &IggyConfig,
    cfg_manager: CfgManager,
) -> Result<(), Box<dyn std::error::Error>> {
    let svc_name = &format!("IMS {exchange_id} Data Service");
    let dbg_print = |msg: &str| {
        if dbg {
            println!("[{svc_name}]: {msg}");
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

        assert!(
            available,
            "Service dependency {d:?} is unavailable; please start it"
        );
    }

    dbg_print("Configure service ip and port automatically!");
    let service_addr = cfg_manager
        .get_svc_socket_addr()
        .await
        .expect("Failed to get service host and port");

    // Free up some memory before starting the service,
    drop(cfg_manager);

    let stream_id = integration_config.control_channel();
    let topic_id = integration_config.control_channel();

    dbg_print("Construct iggy producer client");
    let producer_client = message_shared::build_client(stream_id.clone(), topic_id.clone())
        .await
        .expect("Failed to build client");

    dbg_print("Connecting producer");
    producer_client.connect().await.expect("Failed to connect");

    dbg_print("Login producer");
    producer_client
        .login_user(iggy_config.user().username(), iggy_config.user().password())
        .await
        .expect("Failed to login user");

    dbg_print("Construct iggy consumer");
    let consumer_client = message_shared::build_client(stream_id.clone(), topic_id.clone())
        .await
        .expect("Failed to build client");

    dbg_print("Connecting consumer");
    consumer_client.connect().await.expect("Failed to connect");

    dbg_print("Login consumer");
    consumer_client
        .login_user(iggy_config.user().username(), iggy_config.user().password())
        .await
        .expect("Failed to login user");

    dbg_print("Configuring server and signal handler");
    //Creates a new server
    let server = if dbg {
        Service::with_debug(
            &consumer_client,
            &producer_client,
            integration_config,
            iggy_config,
        )
        .await
        .expect("Failed to build new service")
    } else {
        Service::new(
            &consumer_client,
            &producer_client,
            integration_config,
            iggy_config,
        )
        .await
        .expect("Failed to build new service")
    };

    dbg_print("Set integration online");
    //

    dbg_print("Run server");
    server.run().await.expect("Failed to run service");

    // Print service start header
    print_utils::print_duration("Starting service took:", &start.elapsed());
    print_utils::print_start_header_simple(svc_name, &service_addr);

    #[cfg(unix)]
    let (mut ctrl_c, mut sigterm) = {
        use tokio::signal::unix::{signal, SignalKind};
        (
            signal(SignalKind::interrupt())?,
            signal(SignalKind::terminate())?,
        )
    };

    #[cfg(unix)]
    tokio::select! {
        _ = ctrl_c.recv() => {
            dbg_print("Received SIGINT. Shutting down {NAME} {VERSION}...");
        },
        _ = sigterm.recv() => {
            dbg_print("Received SIGTERM. Shutting down {NAME} {VERSION}...");
        }
    }

    dbg_print("Shutdown server");
    shutdown::shutdown_and_cleanup(dbg, &producer_client, iggy_config)
        .await
        .expect("Failed to shutdown service");
    shutdown::shutdown(&consumer_client)
        .await
        .expect("Failed to shutdown service");

    dbg_print("Set integration offline");
    //

    Ok(())
}
