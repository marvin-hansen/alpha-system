/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::health_check::health_handler::health_handler;
use crate::service::Service;
use common_config::ServiceID;
use common_exchange::ExchangeID;
use common_service::{print_utils, shutdown_utils};
use config_manager::CfgManager;
use imdb_client::*;
use smdb_client::*;
use tokio::time::Instant;
use trait_data_integration::ImsDataIntegration;
use warp::Filter;

mod config;
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
    let cfg_manager = CfgManager::new(
        ServiceID::Default,
        config::ims_data_service_config(exchange_id),
    )
    .await;
    let data_integration = integration_config.integration_id();
    let svc_name = &format!("IMS {data_integration} Service");

    let env = cfg_manager.env_type();
    dbg_print(&format!("Detected Environment: {}", &env));

    dbg_print("get SMDB endpoint from auto config");
    let (smdb_host, smdb_port) = cfg_manager
        .get_smdb_host_port()
        .await
        .expect("Failed to get host and port for DBGW");

    dbg_print("Construct SMDB client");
    let smdb_client = client_utils::select_smdb_client(&env, smdb_host, smdb_port).await;

    dbg_print("get dependencies from auto config");
    let dependencies = cfg_manager.get_service_dependencies();

    dbg_print("Checking whether all dependencies are online");
    for d in &dependencies {
        let available = smdb_client
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
    let imdb_client = client_utils::select_imdb_client(&env, host, port).await;

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

    dbg_print("Configuring health endpoint");
    let port_http = cfg_manager
        .get_ims_data_svc_port(exchange_id)
        .expect("Failed to get port");

    dbg_print("Configure health check route");
    let health_check = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(health_handler);

    dbg_print("Configure service routes");
    let routes = health_check;

    dbg_print("Configure http service");
    let http_signal = shutdown_utils::signal_handler("http server");
    let (_, http_server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], port_http), http_signal);

    dbg_print("Construct server");
    let server = Service::build_service(dbg, ims_integration, integration_config)
        .await
        .expect("Failed to build new service");

    dbg_print("Freeing up memory");
    drop(cfg_manager);
    drop(smdb_client);
    drop(dependencies);

    dbg_print("Starting message service");
    let service_signal = shutdown_utils::signal_handler("messaging server");
    let service_handle = tokio::spawn(server.run(service_signal));

    dbg_print("Starting http server");
    let http_handle = tokio::spawn(http_server);

    // Print service start header
    print_utils::print_duration("Starting service took:", &start.elapsed());
    print_utils::print_start_header_simple(svc_name, &service_addr);
    match tokio::try_join!(http_handle, service_handle) {
        Ok(_) => {}
        Err(e) => {
            println!("IMS Data Integration Service: Failed to start server: {e:?}");
        }
    }

    print_utils::print_stop_header(&ServiceID::Default);

    Ok(())
}
