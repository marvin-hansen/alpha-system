use std::process::exit;
use std::time::{Duration, Instant};

use async_std::task::sleep;
use zenoh::prelude::r#async::*;

use cli::config;

const MAX: i32 = config::MAX_MESSAGES;
const LOW_LATENCY: bool = config::LOW_LATENCY;

#[async_std::main]
async fn main() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        println!("Shutting down...");
        exit(0);
    }).expect("Error setting Ctrl-C handler");

    println!("Build config");
    let config = config::get_config(LOW_LATENCY);
    println!("Opening session...");
    let session = zenoh::open(config).res().await.unwrap();

    let key_expr = String::from("key/expression");
    println!("Declaring Subscriber on '{}'...", &key_expr);

    println!("Declaring Publisher on '{key_expr}'...");
    let publisher = session.declare_publisher(&key_expr).res().await.unwrap();

    let value = String::from("value");

    let start_time = Instant::now();
    for idx in 0..MAX {
        sleep(Duration::from_nanos(10)).await;
        let buf = format!("[{idx:4}] {value}");
        publisher.put(buf).res().await.unwrap();
    }
    let duration = start_time.elapsed();
    println!("Max Messages: {} ", MAX);
    println!("Elapsed time: {:?}", duration);
    let throughput = MAX as f64 / duration.as_secs() as f64;
    println!("Throughput: {:.2} msg/s", throughput);
}
