/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use alpha_socket::{AlphaListener, AlphaSocket};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    io::{self, Write},
    process, thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const ARCHIVER_PATH: &str = "/tmp/alpha_archiver.sock";
const SOCKET_PATH: &str = "/tmp/alpha_server.sock";

#[derive(Debug, Serialize, Deserialize)]
struct TickData {
    symbol: String,
    timestamp: u64,
    price: f64,
    volume: u64,
}

fn main() -> io::Result<()> {
    println!("Starting Tick Data Server...");
    // remove old socket file if exists
    cleanup(SOCKET_PATH);

    // Wait for the archiver to be ready and connect to it using UnixStream
    println!("Waiting for archiver to be ready...");
    let mut attempts = 0;
    let mut archiver = loop {
        match AlphaSocket::connect(ARCHIVER_PATH) {
            Ok(socket) => break socket,
            Err(_) => {
                attempts += 1;
                if attempts == 6 {
                    eprintln!("Failed to connect to archiver after 5 attempts. Exiting.");

                    // remove socket file if exists
                    cleanup(SOCKET_PATH);

                    process::exit(42);
                }
                println!(
                    "Archiver not ready yet. Retrying... (Attempt {}/{})",
                    attempts, 5
                );
                thread::sleep(Duration::from_millis(250));
            }
        }
    };
    println!("Connected to archiver!");

    println!("Setup signal handling to gracefully shutdown on Ctrl-C");
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("Ctrl+C received, initiating shutdown...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Start the main socket listener
    let listener = AlphaListener::bind(SOCKET_PATH)?;
    println!("Server socket created at {}", SOCKET_PATH);

    println!("Waiting for a client connection...");
    loop {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        match listener.accept() {
            Ok(mut socket) => {
                println!("Got a client connection");
                handle_client(&mut socket, &mut archiver)?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Keep waiting for a client connection
                thread::sleep(Duration::from_millis(150));
                continue;
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
                return Err(e);
            }
        }
    }

    // remove socket file if exists
    cleanup(SOCKET_PATH);

    Ok(())
}

// Handle a single client connection
fn handle_client(stream: &mut AlphaSocket, archiver: &mut AlphaSocket) -> io::Result<()> {
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
        let _ = archiver.write_all(tick_data.as_bytes());

        println!("Sent tick data: {:?}", tick);

        // Sleep for a second before sending the next tick
        thread::sleep(Duration::from_micros(500));
    }

    Ok(())
}

fn cleanup(path: &str) {
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(format!("{}.server-write", path));
    let _ = std::fs::remove_file(format!("{}.client-write", path));
}
