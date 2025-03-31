use alpha_socket::{AlphaListener, AlphaSocket};
use std::io::{self, BufRead, BufReader};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;
use std::{process, thread};

const PATH: &str = "/tmp/alpha_archiver.sock";

fn main() -> io::Result<()> {
    println!("Starting Tick Data Archiver...");

    // Remove any existing socket file
    let _ = std::fs::remove_file(PATH);

    // Bind to the socket
    let listener = AlphaListener::bind(PATH)?;
    println!("Listening on {}", PATH);

    println!("Setup signal handling to gracefully shutdown on Ctrl-C");
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("Ctrl+C received, initiating shutdown...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Waiting for client connection...");
    loop {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        match listener.accept() {
            Ok(stream) => handle_client(stream, running.clone())?,
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                thread::sleep(Duration::from_millis(50));
                continue;
            }
            Err(e) => return Err(e),
        }
    }

    println!("Server shutting down...");
    let _ = std::fs::remove_file(PATH);
    process::exit(0);
}

fn handle_client(mut socket: AlphaSocket, running: Arc<AtomicBool>) -> io::Result<()> {
    println!("Connected! Eavesdropping on all messages...");

    // Create a buffered reader for line-by-line reading
    let mut reader = BufReader::new(&mut socket);

    // Read and process tick data
    let mut line = String::new();

    // Read until EOF or shutdown signal
    while running.load(Ordering::SeqCst) {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                // EOF reached
                println!("Connection closed by client");
                break;
            }
            Ok(bytes_read) => {
                if bytes_read > 0 && !line.trim().is_empty() {
                    println!("Archiver: {}", line.trim());
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Non-blocking read, no data available yet
                // Sleep briefly to avoid CPU spinning
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
            Err(e) => {
                println!("Error reading from socket: {}", e);
                break;
            }
        }
    }

    println!("Client handler exiting...");
    Ok(())
}
