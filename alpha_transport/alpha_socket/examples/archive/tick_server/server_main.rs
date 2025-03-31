/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use alpha_socket::{AlphaListener, AlphaSocket};
use serde::{Deserialize, Serialize};
use std::{
    io::{self, Write},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize)]
struct TickData {
    symbol: String,
    timestamp: u64,
    price: f64,
    volume: u64,
}

fn main() -> io::Result<()> {
    println!("Starting Tick Data Server...");

    // Socket paths
    let socket_path = "/tmp/alpha_socket";
    // let archiver_path = "/tmp/alpha_archiver";

    // Create listener (remove old socket file if exists)
    let _ = std::fs::remove_file(socket_path);
    let _ = std::fs::remove_file(format!("{}.server-write", socket_path));
    let _ = std::fs::remove_file(format!("{}.client-write", socket_path));

    // Start the main socket listener
    let listener = AlphaListener::bind(socket_path)?;
    println!("Server socket created at {}", socket_path);

    // Wait for the archiver to be ready and connect to it using UnixStream
    // println!("Waiting for archiver to be ready...");
    // let mut attempts = 0;
    // let archiver = loop {
    //     match AlphaSocket::connect(archiver_path) {
    //         Ok(socket) => break socket,
    //         Err(_) => {
    //             attempts += 1;
    //             if attempts == 6 {
    //                 eprintln!("Failed to connect to archiver after 5 attempts. Exiting.");
    //                 process::exit(42);
    //             }
    //             println!(
    //                 "Archiver not ready yet. Retrying... (Attempt {}/{})",
    //                 attempts, 5
    //             );
    //             thread::sleep(Duration::from_millis(250));
    //         }
    //     }
    // };
    // println!("Connected to archiver!");

    println!("Waiting for a client connection...");
    let mut socket = loop {
        match listener.accept() {
            Ok(stream) => break stream,
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                // Keep waiting for a client connection
                thread::sleep(Duration::from_millis(150));
                continue;
            }
            Err(e) => return Err(e),
        }
    };

    println!("Got a client connection");
    handle_client(&mut socket)?;

    let _ = std::fs::remove_file(format!("{}.server-write", socket_path));
    let _ = std::fs::remove_file(format!("{}.client-write", socket_path));
    let _ = std::fs::remove_file(socket_path);

    Ok(())
}

// Handle a single client connection
fn handle_client(stream: &mut AlphaSocket) -> io::Result<()> {
    println!("Sending tick data...");

    // Generate and send 15 tick data points
    for i in 0..15 {
        // Get current timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Create tick data for Apple stock (AAPL)
        let tick = TickData {
            symbol: "AAPL".to_string(),
            timestamp,
            price: 180.50 + (i as f64 * 0.25), // Increment price
            volume: 1000 + (i * 50),           // Increment volume
        };

        // Serialize tick data to JSON
        let tick_json = serde_json::to_string(&tick)?;
        let tick_data = format!("{}\n", tick_json);

        // Send tick data to client
        stream.write_all(tick_data.as_bytes())?;

        // Also send to archiver
        // let _ = archiver.write_all(tick_data.as_bytes());

        println!("Sent tick data: {:?}", tick);

        // Sleep for a second before sending the next tick
        thread::sleep(Duration::from_micros(500));
    }

    Ok(())
}
