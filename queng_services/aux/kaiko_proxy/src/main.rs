use std::error::Error;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use arc_swap::ArcSwap;
use autometrics::prometheus_exporter;
// https://github.com/purpleprotocol/mimalloc_rust
use mimalloc::MiMalloc;
use tokio::time::Instant;
use tokio_cron_scheduler::{Job, JobScheduler};
use warp::Filter;

use common_config::prelude::ServiceID;
use common_service::{print_utils, shutdown_utils};

use crate::errors::InitError;
use crate::init::InitManager;
use crate::types::meta_data_set::MetaDataSet;
use crate::types::MetaDataStore;

mod errors;
mod fields;
mod handler;
mod init;
mod types;
mod utils;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const UPDATE: bool = false;

const DBG: bool = false;

//  Replace with auto-config.
const PORT_HTTP: u16 = 7777;
const PORT_HEALTH: u16 = 8080;

const SVC_ID: ServiceID = ServiceID::KaikoProxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    dbg_print("Set up prometheus metrics exporter");
    prometheus_exporter::init();

    dbg_print("Load meta-data");
    let meta_data = run_init()
        .await
        .expect("Failed to run init and failed to download metadata");

    dbg_print("Build meta-data store");
    // ArcSwap hot-swaps data in a multi-threaded runtime.
    // https://docs.rs/arc-swap/1.7.1/arc_swap/index.html
    let store: MetaDataStore = Arc::new(ArcSwap::from_pointee(meta_data));

    if UPDATE {
        // New clone of store
        let c = store.clone();

        //  tokio_cron_scheduler
        // https://github.com/mvniekerk/tokio-cron-scheduler
        dbg_print("Build scheduler");
        let scheduler = JobScheduler::new()
            .await
            .expect("Failed to build job scheduler");

        // Run a async update every day at 1 am, EST. (EST = UTC+4)
        //                     sec  min  hour  day   month day of week
        let expression = "0   00    1    *     *     *";
        scheduler
            .add(
                Job::new_async(expression, move |_uuid, _l| {
                    let store = c.clone();
                    Box::pin(async move {
                        dbg_print("Start metadata update");

                        dbg_print("Re-download meta-data");
                        let meta_data = match run_init().await {
                            Ok(res) => res,
                            Err(e) => {
                                eprint!("Updated Error:");
                                eprint!("Updated Error: {}", e);
                                eprint!("Updated Error:");
                                //  notify someone...
                                return;
                            }
                        };

                        // 1) Use hash from existing metadata to determine if anything has changed
                        dbg_print("Load meta-data hash");
                        let guard = store.deref().load();
                        let hash = guard.hash();

                        // 2) If no change, drop the downloaded metadata & do nothing
                        dbg_print("Check meta-data hash");
                        if meta_data.hash() == hash {
                            drop(meta_data);
                            dbg_print("Hash unchanged; no update needed");
                        } else {
                            // 3) if change, update the store with the new metadata
                            dbg_print("Hash changed run update");
                            store.store(Arc::new(meta_data));
                        }
                        dbg_print("Update metadata complete");
                    })
                })
                .expect("Failed to create async update job"),
            )
            .await
            .expect("Failed to add update job to scheduler");

        dbg_print("Start job scheduler");
        scheduler.start().await.expect("Failed to start scheduler");
    }

    //
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

    dbg_print("Configure metrics service");
    // Build http /metrics endpoint
    let routes = warp::get()
        .and(warp::path("metrics"))
        .and(warp::path::end())
        .and_then(handler::get_metrics_handler);

    let (_, metrics_server) = warp::serve(routes).bind_with_graceful_shutdown(
        ([127, 0, 0, 1], PORT_HEALTH),
        shutdown_utils::signal_handler("metrics server"),
    );
    let metrics_handle = tokio::spawn(metrics_server);

    print_update_status(UPDATE);
    print_duration("[main]: Starting server took", &start.elapsed());
    print_utils::print_start_header_simple("Metadata Integration Service", "0.0.0.0:7777/");

    match tokio::try_join!(http_handle, health_handle, metrics_handle) {
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
    let im = InitManager::new(DBG);
    let result = im.init().await;
    drop(im);

    match result {
        Ok(meta_data_set) => Ok(meta_data_set),
        Err(e) => Err(e),
    }
}

pub(crate) fn print_duration(msg: &str, elapsed: &Duration) {
    print_utils::print_duration(msg, elapsed);
}

pub(crate) fn print_update_status(update: bool) {
    if update {
        println!("[main]: Update service online!")
    } else {
        println!("[main]: Update service DISABLED!")
    }
}
