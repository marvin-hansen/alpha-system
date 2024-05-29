use crate::init::InitManager;
use crate::store::Store;
use service_utils::print_utils;
use tokio::time::Instant;
use warp::Filter;

mod errors;
mod fields;
mod init;
mod service;
mod store;
mod types;
mod utils;

const VRB: bool = true;
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

    dbg_print("Build meta-data store");
    let store = Store::new(meta_data);
    let store_filter = warp::any().map(move || store.clone());

    dbg_print("Build assets route");
    let get_assets = warp::get()
        .and(warp::path("assets"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(service::get_assets_from_store);

    dbg_print("Build exchanges route");
    let get_exchanges = warp::get()
        .and(warp::path("exchanges"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(service::get_exchanges_from_store);

    dbg_print("Build instruments route");
    let get_instruments = warp::get()
        .and(warp::path("instruments"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(service::get_instruments_from_store);

    let routes = get_assets.or(get_exchanges).or(get_instruments);

    print_utils::print_duration("[main]: Starting server took", &start.elapsed());

    dbg_print(format!("Start webserver on Port: {}", PORT).as_str());
    warp::serve(routes).run(([0, 0, 0, 0], PORT)).await;
}

fn dbg_print(s: &str) {
    if VRB {
        println!("[main]: {}", s);
    }
}
