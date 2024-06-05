use crate::errors::InitError;
use crate::init::InitManager;
use crate::types::meta_data_set::MetaDataSet;
use crate::types::MetaDataStore;
use arc_swap::ArcSwap;
use service_utils::print_utils;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use tokio_cron::{daily, hourly, Job, Scheduler};
use warp::Filter;

mod errors;
mod fields;
mod handler;
mod init;
mod types;
mod utils;

const VRB: bool = false;
const PORT: u16 = 7777;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    dbg_print("Load meta-data");
    let meta_data = run_init()
        .await
        .expect("Failed to run init and failed to download metadata");

    dbg_print("Build meta-data store");
    // ArcSwap hot-swaps data in a multi-threaded runtime.
    // https://docs.rs/arc-swap/1.7.1/arc_swap/index.html
    let store: MetaDataStore = Arc::new(ArcSwap::from_pointee(meta_data.clone()));
    let c = store.clone();
    let with_state = warp::any().map(move || store.clone());

    dbg_print("Build scheduler");
    // https://github.com/kurtbuilds/tokio-cron
    let mut scheduler = Scheduler::utc();

    // Run a named async closure "update metadata" every day at 1am UTC.
    scheduler.add(Job::named("update metadata", hourly("00"), move || {
        // remove
        println!("Start metadata update");

        dbg_print("Start metadata update");
        let store = c.clone();
        async move {
            dbg_print("Re-download meta-data");
            let meta_data = match run_init().await {
                Ok(res) => res,
                Err(e) => {
                    eprint!("Updated Error: {}", e);
                    // Send a message to notify someone.
                    return;
                }
            };

            // 1) Use hash from metadata to determine if anything has changed
            dbg_print("Load meta-data hash");
            let guard = store.deref().load();
            let hash = guard.hash();

            dbg_print("Check meta-data hash");
            // 2) If no change, drop the downloaded metadata & do nothing
            if meta_data.hash() == hash {
                drop(meta_data);
                dbg_print("Hash unchanged; no update needed");
            } else {
                // 3) if change, update the store with the new metadata
                dbg_print("Hash changed run update");
                store.store(Arc::new(meta_data));
            }
            dbg_print("Update metadata complete");

            // remove
            println!("Update metadata complete");
        }
    }));

    dbg_print("Build health route");
    let health_check = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(handler::get_health_handler);

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

    let routes = health_check
        .or(get_assets)
        .or(get_exchanges)
        .or(get_instruments)
        .or(get_stats);

    print_duration("[main]: Starting server took", &start.elapsed());
    print_utils::print_start_header_simple("Metadata Integration Service", "0.0.0.0:7777/");
    warp::serve(routes).run(([0, 0, 0, 0], PORT)).await;
}

fn dbg_print(s: &str) {
    if VRB {
        println!("[main]: {}", s);
    }
}

async fn run_init() -> Result<MetaDataSet, InitError> {
    // im drops at the end of the function & released all temporary memory,
    let im = InitManager::new(VRB);
    return match im.init().await {
        Ok(meta_data_set) => Ok(meta_data_set),
        Err(e) => Err(e),
    };
}

fn print_duration(msg: &str, elapsed: &Duration) {
    if VRB {
        print_utils::print_duration(msg, elapsed);
    }
}
