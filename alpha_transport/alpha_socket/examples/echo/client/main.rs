use alpha_socket::AlphaStream;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

const PATH: &str = "/tmp/echo.sock";

fn main() -> std::io::Result<()> {
    // Wait a bit to ensure server is ready
    thread::sleep(Duration::from_millis(100));

    // Connect to the socket
    let mut socket = AlphaStream::connect(PATH)?;

    // Write to the socket
    let message = b"Hello world";
    socket.write_all(message)?;
    socket.flush()?;

    // Read from the socket with retry logic
    let mut buf = [0u8; 1024];
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

    // Close the socket
    socket.shutdown()?;

    Ok(())
}
