use alpha_socket::{AlphaListener, AlphaSocket};
use std::io::{self, BufRead, BufReader};
use std::process;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

fn main() -> io::Result<()> {
    println!("Starting Tick Data Archiver...");

    // Socket path
    let archiver_path = "/tmp/alpha_archiver";

    // Remove any existing socket file
    let _ = std::fs::remove_file(archiver_path);

    // Bind the socket using AlphaListener
    println!("Binding to archiver socket at {}...", archiver_path);
    let listener = AlphaListener::bind(archiver_path)?;
    println!("Archiver socket created at {}", archiver_path);

    // Setup signal handling to gracefully shutdown on Ctrl-C
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("Ctrl+C received, initiating shutdown...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Accept connections in the main thread to ensure it's ready immediately
    println!("Accept connections...");
    for stream in listener.incoming() {
        if !running.load(Ordering::SeqCst) {
            println!("Shutting down server...");
            break;
        }
        match stream {
            Ok(stream) => {
                // Pass the running flag to the client handler
                let client_running = running.clone();
                handle_client(stream, client_running)
                    .unwrap_or_else(|error| eprintln!("{:?}", error));
            }
            Err(err) => {
                eprintln!("Error accepting connection: {:?}", err);
            }
        }
    }

    println!("Got it! Exiting...");
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
