/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

// https://github.com/purpleprotocol/mimalloc_rust
use mimalloc::MiMalloc;
use tokio::sync::RwLock;
use tokio::time::Instant;
use warp::Filter;

use common_config::ServiceID;
use common_errors::InitError;
use common_metadata::MetaDataSet;
use common_service::{print_utils, shutdown_utils};

mod handler;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = false;

const PORT_HTTP: u16 = 7777;

const SVC_ID: ServiceID = ServiceID::KaikoProxy;

pub(crate) type MetaDataStore = Arc<RwLock<MetaDataSet>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    dbg_print("Download meta-data");
    let meta_data = run_init()
        .await
        .expect("Failed to run init and failed to download metadata");

    dbg_print("Build in-memory meta-data store");
    let store: MetaDataStore = Arc::new(RwLock::new(meta_data));

    let with_state = warp::any().map(move || store.clone());

    dbg_print("Build assets route");
    let get_assets = warp::get()
        .and(warp::path("assets"))
        .and(warp::path::end())
        .and(with_state.clone())
        .and_then(handler::get_assets_handler);

    dbg_print("Build exchanges route");
    let get_exchanges = warp::get()
        .and(warp::path("exchanges"))
        .and(warp::path::end())
        .and(with_state.clone())
        .and_then(handler::get_exchanges_handler);

    dbg_print("Build instruments route");
    let get_instruments = warp::get()
        .and(warp::path("instruments"))
        .and(warp::path::end())
        .and(with_state.clone())
        .and_then(handler::get_instruments_handler);

    dbg_print("Build stats route");
    let get_stats = warp::get()
        .and(warp::path("stats"))
        .and(warp::path::end())
        .and(with_state.clone())
        .and_then(handler::get_stats_handler);

    dbg_print("Configure health check route");
    let health_check = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(handler::get_health_handler);

    dbg_print("Configure service routes");
    let routes = health_check
        .or(get_assets)
        .or(get_exchanges)
        .or(get_instruments)
        .or(get_stats);

    dbg_print("Configure http service");
    let http_signal = shutdown_utils::signal_handler("http server");
    let (_, http_server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], PORT_HTTP), http_signal);

    let http_handle = tokio::spawn(http_server);

    print_duration("[main]: Starting server took", &start.elapsed());
    print_utils::print_start_header_simple("Metadata Integration Service", "0.0.0.0:7777/");

    match tokio::try_join!(http_handle) {
        Ok(_) => {}
        Err(e) => {
            println!("Kaiko Proxy: Failed to start HTTP server: {e:?}");
        }
    }

    print_utils::print_stop_header(&SVC_ID);
    Ok(())
}

pub(crate) fn dbg_print(s: &str) {
    if DBG {
        println!("[main]: {s}");
    }
}

pub(crate) async fn run_init() -> Result<MetaDataSet, InitError> {
    // Download meta-data;
    let result = kaiko_download::download_meta_data(DBG, true).await;
    match result {
        Ok(meta_data_set) => Ok(meta_data_set),
        Err(e) => Err(e),
    }
}

pub(crate) fn print_duration(msg: &str, elapsed: &Duration) {
    print_utils::print_duration(msg, elapsed);
}
