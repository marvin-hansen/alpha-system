use std::process::exit;

use cli::config;
use futures::select;
use zenoh::prelude::r#async::*;
const KEY_EXPRESS: &str = config::KEY_EXPRESS;
const LOW_LATENCY: bool = config::LOW_LATENCY;
const MAX: i32 = config::MAX_MESSAGES;

#[async_std::main]
async fn main() {
    let mut count = 0;

    ctrlc::set_handler(move || {
        println!("Shutting down...");
        println!("Count: {}", count);
        println!("Max Count: {}", MAX);

        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Build config");
    let config = config::get_config(LOW_LATENCY);

    println!("Opening session...");
    let session = zenoh::open(config).res().await.unwrap();

    println!("Declaring Subscriber on '{}'...", KEY_EXPRESS);
    let subscriber = session.declare_subscriber(KEY_EXPRESS).res().await.unwrap();

    println!("Enter 'Ctrl+C' to quit...");
    loop {
        select!(
            _sample = subscriber.recv_async() => {
            count += 1;
            },
        );
    }
}
