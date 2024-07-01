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
use tokio_cron_scheduler::{Job, JobScheduler};
use warp::Filter;

// https://github.com/purpleprotocol/mimalloc_rust
use mimalloc::MiMalloc;

mod errors;
mod fields;
mod handler;
mod init;
mod types;
mod utils;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const VRB: bool = false;
const PORT: u16 = 7777;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    dbg_print("Load meta-data");
    let meta_data = run_init(false)
        .await
        .expect("Failed to run init and failed to download metadata");

    dbg_print("Build meta-data store");
    // ArcSwap hot-swaps data in a multi-threaded runtime.
    // https://docs.rs/arc-swap/1.7.1/arc_swap/index.html
    let store: MetaDataStore = Arc::new(ArcSwap::from_pointee(meta_data.clone()));
    let c = store.clone();
    let with_state = warp::any().map(move || store.clone());

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
                    let meta_data = match run_init(true).await {
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

    dbg_print("Configure service routes");
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

async fn run_init(update: bool) -> Result<MetaDataSet, InitError> {
    let im = InitManager::new(VRB);
    let result = im.init(update).await;
    drop(im);

    return match result {
        Ok(meta_data_set) => Ok(meta_data_set),
        Err(e) => Err(e),
    };
}

fn print_duration(msg: &str, elapsed: &Duration) {
    print_utils::print_duration(msg, elapsed);
}
