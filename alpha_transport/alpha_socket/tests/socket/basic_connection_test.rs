use alpha_socket::AlphaListener;
use alpha_socket::AlphaStream;
use std::io::{Read, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tempfile::tempdir;

#[test]
fn test_basic_connection() {
    let dir = tempdir().unwrap();
    let socket_path = dir.path().join("basic.sock");
    let socket_path_clone = socket_path.clone();

    let server_ready = Arc::new(AtomicBool::new(false));
    let client_ready = Arc::new(AtomicBool::new(false));
    let server_ready_clone = server_ready.clone();
    let client_ready_clone = client_ready.clone();

    // Start server thread
    let server = thread::spawn(move || {
        println!("Server: Creating socket at {:?}", socket_path_clone);
        let listener = AlphaListener::bind(&socket_path_clone).unwrap();

        // Signal that server is ready
        server_ready_clone.store(true, Ordering::SeqCst);
        println!("Server: Socket bound, signaled ready");

        // Wait for client connection
        println!("Server: Waiting for client connection...");
        let mut stream = listener.accept().unwrap();
        println!("Server: Client connected!");

        // Wait for client to be ready to send data
        while !client_ready_clone.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(10));
        }
        println!("Server: Client signaled ready to send data");

        // Read data from client
        let mut buffer = [0u8; 32];
        let mut total_read = 0;
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 50;

        while total_read == 0 && attempts < MAX_ATTEMPTS {
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 {
                        println!(
                            "Server: Buffer is empty, attempt {}/{}",
                            attempts, MAX_ATTEMPTS
                        );
                        // thread::sleep(Duration::from_millis(10));
                        attempts += 1;
                    } else {
                        total_read = size;
                        let received_message = &buffer[..size];
                        println!("Server: Received message: {:?}", received_message);

                        // Echo the message back
                        stream.write_all(received_message).unwrap();
                        println!("Server: Echoed message back to client");
                        break;
                    }
                }
                Err(e) => {
                    println!("Server: Read error: {:?}", e);
                    thread::sleep(Duration::from_millis(10));
                    attempts += 1;
                }
            }
        }

        assert!(total_read > 0, "Server failed to read message from client");
    });

    // Give server time to start up
    thread::sleep(Duration::from_millis(100));

    // Start client thread
    let client = thread::spawn(move || {
        // Wait for server to be ready
        println!("Client: Waiting for server to be ready...");
        while !server_ready.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(10));
        }

        println!(
            "Client: Server is ready, socket exists: {}",
            socket_path.exists()
        );
        assert!(socket_path.exists(), "Socket file doesn't exist");

        // Connect to server
        println!("Client: Connecting to socket...");
        let mut stream = AlphaStream::connect(&socket_path).unwrap();
        println!("Client: Connected successfully");

        // Prepare message
        let message = b"Hello, AlphaSocket!";

        // Signal client is ready to send data
        client_ready.store(true, Ordering::SeqCst);
        println!("Client: Signaled ready to send data");

        // Send message
        println!("Client: Sending message: {:?}", message);
        stream.write_all(message).unwrap();
        stream.flush().unwrap();
        println!("Client: Message sent successfully");

        // Read response
        let mut buffer = [0u8; 32];
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 50;

        while attempts < MAX_ATTEMPTS {
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size > 0 {
                        let received_message = &buffer[..size];
                        println!("Client: Received response: {:?}", received_message);
                        assert_eq!(
                            received_message, message,
                            "Client received incorrect response"
                        );
                        break;
                    } else {
                        println!(
                            "Client: Empty response, attempt {}/{}",
                            attempts, MAX_ATTEMPTS
                        );
                    }
                }
                Err(e) => {
                    println!("Client: Read error: {:?}", e);
                }
            }

            attempts += 1;
            thread::sleep(Duration::from_millis(10));
        }

        assert!(
            attempts < MAX_ATTEMPTS,
            "Client timed out waiting for response"
        );
    });

    // Wait for both threads to complete
    server.join().unwrap();
    client.join().unwrap();

    println!("Test completed successfully");
}
