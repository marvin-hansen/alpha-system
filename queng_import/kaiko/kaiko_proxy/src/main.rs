use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

// https://github.com/purpleprotocol/mimalloc_rust
use mimalloc::MiMalloc;
use tokio::sync::RwLock;
use tokio::time::Instant;
use warp::Filter;

use common_config::prelude::ServiceID;
use common_errors::prelude::InitError;
use common_metadata::prelude::MetaDataSet;
use common_service::{print_utils, shutdown_utils};

mod handler;
mod health;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const DBG: bool = false;

//  Replace with auto-config.
const PORT_HTTP: u16 = 7777;
const PORT_HEALTH: u16 = 8080;

const SVC_ID: ServiceID = ServiceID::KaikoProxy;

pub(crate) type MetaDataStore = Arc<RwLock<MetaDataSet>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    dbg_print("Load meta-data");
    let meta_data = run_init()
        .await
        .expect("Failed to run init and failed to download metadata");

    dbg_print("Build meta-data store");
    // ArcSwap hot-swaps data in a multi-threaded runtime.
    // https://docs.rs/arc-swap/1.7.1/arc_swap/index.html
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

    dbg_print("Configure service routes");
    let routes = get_assets
        .or(get_exchanges)
        .or(get_instruments)
        .or(get_stats);

    dbg_print("Configure http service");
    let http_signal = shutdown_utils::signal_handler("http server");
    let (_, http_server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], PORT_HTTP), http_signal);

    let http_handle = tokio::spawn(http_server);

    dbg_print("Configure health check route");
    let health_check = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(handler::get_health_handler);

    dbg_print("Configure health check service");
    let health_signal = shutdown_utils::signal_handler("http server");
    let (_, health_server) = warp::serve(health_check)
        .bind_with_graceful_shutdown(([127, 0, 0, 1], PORT_HEALTH), health_signal);
    let health_handle = tokio::spawn(health_server);

    print_duration("[main]: Starting server took", &start.elapsed());
    print_utils::print_start_header_simple("Metadata Integration Service", "0.0.0.0:7777/");

    match tokio::try_join!(http_handle, health_handle) {
        Ok(_) => {}
        Err(e) => {
            println!("Kaiko Proxy: Failed to start HTTP server: {:?}", e);
        }
    }

    print_utils::print_stop_header(&SVC_ID);
    Ok(())
}

pub(crate) fn dbg_print(s: &str) {
    if DBG {
        println!("[main]: {}", s);
    }
}

pub(crate) async fn run_init() -> Result<MetaDataSet, InitError> {
    let result = kaiko_download::download_meta_data(DBG, None).await;

    match result {
        Ok(meta_data_set) => Ok(meta_data_set),
        Err(e) => Err(e),
    }
}

pub(crate) fn print_duration(msg: &str, elapsed: &Duration) {
    print_utils::print_duration(msg, elapsed);
}
