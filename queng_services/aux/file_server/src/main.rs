use crate::init::InitManager;
use crate::types::MetaDataStore;
use arc_swap::ArcSwap;
use service_utils::print_utils;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use warp::Filter;

mod errors;
mod fields;
mod handler;
mod init;
mod types;
mod utils;

const VRB: bool = false;
const PORT: u16 = 7777;

// Inspired by a log rocket article
// Building a REST API in Rust with warp
// https://blog.logrocket.com/building-rest-api-rust-warp/

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let im = InitManager::new(VRB);
    let meta_data = im
        .init()
        .await
        .expect("Failed to initialize FileServer service.");

    // Free up memory.
    drop(im);

    dbg_print("Build meta-data store");
    // ArcSwap hot-swaps data in a multi-threaded runtime.
    // https://docs.rs/arc-swap/1.7.1/arc_swap/index.html
    let store: MetaDataStore = Arc::new(ArcSwap::from_pointee(meta_data.clone()));
    let with_state = warp::any().map(move || store.clone());

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

    dbg_print("Build symbol mapping route");
    let get_symbol_mapping = warp::get()
        .and(warp::path("symbol_mapping"))
        .and(warp::path::end())
        .and(with_state.clone())
        .and_then(handler::get_symbol_mapping_handler);

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
        .or(get_symbol_mapping)
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

fn print_duration(msg: &str, elapsed: &Duration) {
    if VRB {
        print_utils::print_duration(msg, elapsed);
    }
}
