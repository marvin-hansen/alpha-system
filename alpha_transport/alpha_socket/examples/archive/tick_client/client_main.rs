/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use alpha_socket::AlphaSocketWithArchiver;
use serde::{Deserialize, Serialize};
use std::{
    io::{self, BufRead, BufReader},
    thread,
    time::Duration,
};

#[derive(Debug, Serialize, Deserialize)]
struct TickData {
    symbol: String,
    timestamp: u64,
    price: f64,
    volume: u64,
}

fn main() -> io::Result<()> {
    println!("Starting Tick Data Client...");

    // Socket path
    let socket_path = "/tmp/alpha_socket";

    // Connect to server
    println!("Connecting to server at {}...", socket_path);
    let socket = alpha_socket::AlphaSocket::connect(socket_path)?;

    // Create AlphaSocketWithArchiver without archiver for client
    let socket_with_archiver = AlphaSocketWithArchiver::new(socket, None);

    // Create a buffered reader for line-by-line reading
    let mut reader = BufReader::new(socket_with_archiver);

    println!("Connected! Waiting for tick data...");

    // Read and process tick data
    let mut line = String::new();
    let mut count = 0;

    // Read until we receive 15 data points or EOF
    while count < 15 {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                // EOF reached
                println!("Connection closed by server");
                break;
            }
            Ok(_) => {
                // Parse JSON
                match serde_json::from_str::<TickData>(&line.trim()) {
                    Ok(tick) => {
                        println!(
                            "Received tick: Symbol: {}, Price: ${:.2}, Volume: {}",
                            tick.symbol, tick.price, tick.volume
                        );
                        count += 1;
                    }
                    Err(e) => {
                        println!("Failed to parse tick data: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error reading from socket: {}", e);
                break;
            }
        }
    }

    println!("Received all expected tick data!");

    // Keep running to allow viewing the results
    thread::sleep(Duration::from_secs(2));

    Ok(())
}
