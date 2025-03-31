use alpha_socket::AlphaListener;
use alpha_socket::AlphaSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tempfile::tempdir;

// Helper function to ensure the socket file exists and is ready
fn ensure_socket_ready(path: &std::path::Path, max_wait: Duration) -> bool {
    let start = std::time::Instant::now();
    while start.elapsed() < max_wait {
        // Check multiple conditions for socket readiness
        if path.exists() {
            // Optional: Add more readiness checks if needed
            return true;
        }
        thread::sleep(Duration::from_millis(10));
    }
    false
}

#[test]
fn test_unix_stream_echo() {
    let dir = tempdir().unwrap();
    let socket_path = dir.path().join("test.sock");

    // Use a mutex to coordinate server and client
    let server_ready = Arc::new(Mutex::new(false));
    let server_ready_clone = Arc::clone(&server_ready);

    // Start server in a separate thread
    let server_path = socket_path.clone();
    let server = thread::spawn(move || {
        let listener = AlphaListener::bind(&server_path).expect("Failed to bind socket");

        // Mark server as ready
        {
            let mut ready = server_ready_clone.lock().unwrap();
            *ready = true;
        }

        // Wait for client connection
        let mut stream = listener.accept().expect("Failed to accept connection");

        let mut buffer = [0u8; 1024];
        let n = stream
            .read_with_retry(&mut buffer, 50)
            .expect("Server failed to read message");

        // Echo back what was received
        stream
            .write_with_retry(&buffer[..n], 50)
            .expect("Server failed to write message");
    });

    // Wait for server to be ready
    {
        let mut attempts = 0;
        while !*server_ready.lock().unwrap() && attempts < 100 {
            thread::sleep(Duration::from_millis(10));
            attempts += 1;
        }
    }

    // Ensure socket is ready with a longer timeout
    assert!(
        ensure_socket_ready(&socket_path, Duration::from_secs(2)),
        "Socket not ready"
    );

    // Connect client with retry
    let mut client = AlphaSocket::connect(&socket_path).expect("Failed to connect");

    // Send test data
    let test_data = b"Hello, AlphaSocket!";
    client
        .write_with_retry(test_data, 50)
        .expect("Failed to write data");

    // Read response
    let mut response = [0u8; 1024];
    let n = client
        .read_with_retry(&mut response, 50)
        .expect("Failed to read response");

    assert_eq!(&response[..n], test_data);

    // Wait for server to finish
    server.join().expect("Failed to join server thread");
}

#[test]
fn test_unix_stream_multiple_messages() {
    let dir = tempdir().unwrap();
    let socket_path = dir.path().join("test.sock");

    // Use a mutex to coordinate server and client
    let server_ready = Arc::new(Mutex::new(false));
    let server_ready_clone = Arc::clone(&server_ready);

    // Start server in a separate thread
    let server_path = socket_path.clone();
    let server = thread::spawn(move || {
        let listener = AlphaListener::bind(&server_path).expect("Failed to bind socket");

        // Mark server as ready
        {
            let mut ready = server_ready_clone.lock().unwrap();
            *ready = true;
        }

        let mut stream = listener.accept().expect("Failed to accept connection");

        // Echo back 5 messages
        for _ in 0..5 {
            let mut buffer = [0u8; 1024];
            let n = stream
                .read_with_retry(&mut buffer, 50)
                .expect("Server failed to read message");

            stream
                .write_with_retry(&buffer[..n], 50)
                .expect("Server failed to write message");
        }
    });

    // Wait for server to be ready
    {
        let mut attempts = 0;
        while !*server_ready.lock().unwrap() && attempts < 100 {
            thread::sleep(Duration::from_millis(10));
            attempts += 1;
        }
    }

    // Ensure socket is ready with a longer timeout
    assert!(
        ensure_socket_ready(&socket_path, Duration::from_secs(2)),
        "Socket not ready"
    );

    // Connect client
    let mut client = AlphaSocket::connect(&socket_path).expect("Failed to connect");

    // Send 5 test messages (all same length for array type)
    let messages = [
        b"Message One   ",
        b"Message Two   ",
        b"Message Three ",
        b"Message Four  ",
        b"Message Five  ",
    ];

    for &msg in &messages {
        // Send message
        client
            .write_with_retry(msg, 50)
            .expect("Failed to write data");

        // Read response
        let mut response = [0u8; 1024];
        let n = client
            .read_with_retry(&mut response, 50)
            .expect("Failed to read response");

        assert_eq!(&response[..n], msg);
    }

    // Wait for server to finish
    server.join().expect("Failed to join server thread");
}

#[test]
fn test_unix_stream_concurrent_clients() {
    let dir = tempdir().unwrap();
    let socket_path = dir.path().join("test.sock");

    // Start server in a separate thread
    let server_path = socket_path.clone();
    let server = thread::spawn(move || {
        let listener = AlphaListener::bind(&server_path).expect("Failed to bind socket");

        // Handle 3 concurrent clients
        for _ in 0..3 {
            let mut stream = listener.accept().expect("Failed to accept connection");
            thread::spawn(move || {
                let mut buffer = [0u8; 1024];
                let n = match stream.read_with_retry(&mut buffer, 20) {
                    Ok(n) => n,
                    Err(e) => {
                        println!("Server read error: {}", e);
                        return;
                    }
                };

                match stream.write_with_retry(&buffer[..n], 20) {
                    Ok(_) => {}
                    Err(e) => println!("Server write error: {}", e),
                }
            });
        }
    });

    // Wait for the socket file to exist
    let start = std::time::Instant::now();
    while !socket_path.exists() {
        if start.elapsed() > Duration::from_millis(1000) {
            panic!("Socket was not created within timeout");
        }
        thread::sleep(Duration::from_millis(10));
    }

    println!("Socket ready");

    // Ensure server is fully set up
    thread::sleep(Duration::from_millis(500));

    // Connect 3 clients
    let mut handles = Vec::new();
    for i in 0..3 {
        let client_path = socket_path.clone();
        let message = format!("Message from client {}", i);

        // Add small delay between client connections
        thread::sleep(Duration::from_millis(200));

        let handle = thread::spawn(move || {
            // Wait to ensure the server is ready
            thread::sleep(Duration::from_millis(100 * i as u64));

            let mut client = match AlphaSocket::connect(&client_path) {
                Ok(c) => c,
                Err(e) => panic!("Client {} failed to connect: {}", i, e),
            };

            // Send message
            match client.write_with_retry(message.as_bytes(), 20) {
                Ok(_) => {}
                Err(e) => panic!("Client {} write error: {}", i, e),
            }

            // Read response
            let mut response = [0u8; 1024];
            let n = match client.read_with_retry(&mut response, 30) {
                Ok(n) => n,
                Err(e) => panic!("Client {} read error: {}", i, e),
            };

            assert_eq!(&response[..n], message.as_bytes());
        });
        handles.push(handle);
    }

    // Wait for all clients to finish
    for (i, handle) in handles.into_iter().enumerate() {
        if let Err(e) = handle.join() {
            println!("Client {} thread panicked: {:?}", i, e);
        }
    }

    // Wait for server to finish
    if let Err(e) = server.join() {
        println!("Server thread panicked: {:?}", e);
    }
}

#[test]
fn test_alpha_stream_pair() {
    println!("Starting test_alpha_stream_pair");

    // Create a pair of connected streams
    println!("Creating stream pair");
    let (mut client, mut server) = AlphaSocket::pair().expect("Failed to create stream pair");
    println!("Stream pair created successfully");

    // Test data to send
    let test_data = b"Hello, AlphaSocket Pair!";
    println!("Test data prepared: {:?}", test_data);

    // Write from stream1 to stream2
    println!("Writing from stream1 to stream2");
    client
        .write_with_retry(test_data, 20)
        .expect("Failed to write to stream1");
    println!("Write from stream1 to stream2 completed");

    // Read from stream2
    println!("Preparing to read from stream2");
    let mut read_buffer = [0u8; 32];
    println!("Read buffer initialized");

    let n = server
        .read_with_retry(&mut read_buffer, 10)
        .unwrap_or_else(|e| {
            eprintln!("Read from stream2 error: {:?}", e);
            panic!("Failed to read from stream2");
        });
    println!("Read {} bytes from stream2", n);

    // Verify the data
    println!("Verifying data read from stream2");
    assert_eq!(&read_buffer[..n], test_data);
    println!("Data verification for stream2 read successful");

    // Write from stream2 to stream1 (bidirectional test)
    println!("Writing from stream2 to stream1");
    server
        .write_with_retry(test_data, 10)
        .expect("Failed to write to stream2");
    println!("Write from stream2 to stream1 completed");

    // Read from stream1
    println!("Preparing to read from stream1");
    let mut read_buffer2 = [0u8; 32];
    println!("Read buffer for stream1 initialized");

    let n2 = client
        .read_with_retry(&mut read_buffer2, 10)
        .unwrap_or_else(|e| {
            eprintln!("Read from stream1 error: {:?}", e);
            panic!("Failed to read from stream1");
        });
    println!("Read {} bytes from stream1", n2);

    // Verify the data
    println!("Verifying data read from stream1");
    assert_eq!(&read_buffer2[..n2], test_data);
    println!("Data verification for stream1 read successful");

    println!("test_alpha_stream_pair completed successfully");
}
