use crate::service::Service;
use common_config::ServiceID;
use common_exchange::ExchangeID;
use common_service::print_utils;
use config_manager::CfgManager;
use iggy::client::{Client, UserClient};
use imdb_client::IMDBClient;
use imdb_client::ImdbClientTrait;
use smdb_client::*;
use tokio::time::Instant;
use trait_data_integration::ImsDataIntegration;

mod config;
mod handle;
mod health_check;
mod run;
mod service;
mod shutdown;
mod utils;

pub async fn start<Integration>(
    dbg: bool,
    exchange_id: ExchangeID,
    ims_integration: Integration,
) -> Result<(), Box<dyn std::error::Error>>
where
    Integration: ImsDataIntegration + Send + Sync + 'static,
{
    let dbg_print = |msg: &str| {
        if dbg {
            println!("[{exchange_id}]: {msg}");
        }
    };
    //
    let start = Instant::now();
    //
    dbg_print("build config files");
    let integration_config = &config::ims_data_integration_config(exchange_id);
    let iggy_config = &config::ims_data_iggy_config(exchange_id);
    let cfg_manager = CfgManager::new(
        ServiceID::Default,
        config::ims_data_service_config(exchange_id),
    )
    .await;
    let data_integration = integration_config.integration_id();
    let svc_name = &format!("IMS {data_integration} Service");

    dbg_print("get SMDB endpoint from auto config");
    let (smdb_host, smdb_port) = cfg_manager
        .get_smdb_host_port()
        .await
        .expect("Failed to get host and port for DBGW");

    dbg_print("get dependencies from auto config");
    let dependencies = cfg_manager.get_service_dependencies();

    dbg_print("Checking whether all dependencies are online");
    let smdb_manager = SMDBClient::new(smdb_host, smdb_port).await;
    for d in &dependencies {
        let available = smdb_manager
            .check_if_service_id_exists(*d)
            .await
            .expect(" Failed to check if service dependency exists");

        assert!(
            available,
            "Service dependency {d:?} is unavailable; please start it"
        );
    }

    dbg_print("Configure IMDB client");
    let (host, port) = cfg_manager
        .get_imdb_host_port()
        .await
        .expect("Failed to get MDDB host");

    dbg_print("Construct IMDB client");
    let imdb_client = IMDBClient::new(host, port)
        .await
        .expect("Failed to create IMDB client");

    dbg_print("Get integration form IMDB");
    let integration_id = "ims-data-binance".to_string();
    let exists = imdb_client
        .check_if_integration_exists(integration_id.clone())
        .await
        .expect("Failed to get integration");
    if !exists {
        panic!("Integration {integration_id} does not exist on IMDB");
    }

    dbg_print("Configure service ip and port automatically!");
    let service_addr = cfg_manager
        .get_svc_socket_addr()
        .await
        .expect("Failed to get service host and port");

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

    dbg_print("Configuring metrics endpoint");
    let (metrics_addr, _) = cfg_manager
        .get_metrics_socket_addr_uri()
        .expect("DBGW: Failed to get metric host, uri, and port");
    dbg_print(&metrics_addr);

    dbg_print("Construct health endpoint server");
    let http_server = health_check::health_handler::get_http_health_server(metrics_addr).await;

    dbg_print("Construct server");
    let server = if dbg {
        Service::with_debug(
            &consumer_client,
            &producer_client,
            ims_integration,
            integration_config,
            iggy_config,
        )
        .await
        .expect("Failed to build new service")
    } else {
        Service::new(
            &consumer_client,
            &producer_client,
            ims_integration,
            integration_config,
            iggy_config,
        )
        .await
        .expect("Failed to build new service")
    };

    dbg_print("Set integration online on IMDB");
    imdb_client
        .set_integration_online(integration_id.clone())
        .await
        .expect("Failed to set integration online");

    // Free up some memory before starting the service,
    drop(cfg_manager);
    drop(smdb_manager);
    drop(dependencies);

    // Print service start header
    print_utils::print_duration("Starting service took:", &start.elapsed());
    print_utils::print_start_header_simple(svc_name, &service_addr);

    server.run().await.expect("Failed to run service");
    http_server.await;

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

    dbg_print("Shutdown iggy producer");
    dbg_print("Deleting streams and topics");
    message_shared::cleanup(&producer_client, iggy_config)
        .await
        .expect("Failed to clean up iggy");

    dbg_print("Logging out user");
    message_shared::logout_user(&producer_client)
        .await
        .expect("Failed to logout user");

    dbg_print("Shutting down iggy client");
    message_shared::shutdown(&producer_client)
        .await
        .expect("Failed to shutdown iggy consumer");

    dbg_print("Shutdown iggy consumer");
    message_shared::shutdown(&consumer_client)
        .await
        .expect("Failed to shutdown iggy consumer");

    dbg_print("Set integration offline on IMDB");
    imdb_client
        .set_integration_offline(integration_id.clone())
        .await
        .expect("Failed to set integration offline on IMDB");

    Ok(())
}
