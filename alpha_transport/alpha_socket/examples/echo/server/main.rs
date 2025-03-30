use alpha_socket::AlphaListener;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

const PATH: &str = "/tmp/echo.sock";
const TIMEOUT_MS: u64 = 5000; // 5 seconds timeout

fn main() -> std::io::Result<()> {
    // Remove any existing socket file
    let _ = std::fs::remove_file(PATH);

    // Bind to the socket
    let listener = AlphaListener::bind(PATH)?;
    println!("Listening on {}", PATH);

    // Wait for a client connection with a timeout
    println!("Waiting for client connection...");

    let start = std::time::Instant::now();
    let mut socket = loop {
        match listener.accept() {
            Ok(stream) => break stream,
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                if start.elapsed() > Duration::from_millis(TIMEOUT_MS) {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "No client connection within timeout period",
                    ));
                }
                thread::sleep(Duration::from_millis(50));
                continue;
            }
            Err(e) => return Err(e),
        }
    };

    println!("Got a client connection");

    // Prepare buffer
    let mut buf = [0u8; 1024];

    // Read from the socket with retry logic
    let size = loop {
        match socket.read(&mut buf) {
            Ok(0) => {
                thread::sleep(Duration::from_millis(50));
                continue;
            }
            Ok(size) => break size,
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(50));
                continue;
            }
            Err(e) => return Err(e),
        }
    };

    println!("Received: {}", String::from_utf8_lossy(&buf[..size]));

    // Write back to the socket i.e. echo
    socket.write_all(&buf[..size])?;
    socket.flush()?;

    // Close the socket
    socket.shutdown()?;

    Ok(())
}
