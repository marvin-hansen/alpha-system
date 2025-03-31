/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use alpha_socket::{AlphaSocket, AlphaSocketWithArchiver};
use std::io::{Read, Write};
use std::path::Path;
use std::{fs, io};
use tempfile::tempdir;

// Helper function to create temporary socket and archiver paths
fn setup_temp_paths() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let socket_path = temp_dir.path().join("test_socket");
    let archiver_path = temp_dir.path().join("test_archiver");

    // Create placeholder files
    {
        let _ = fs::File::create(&socket_path).expect("Failed to create test socket file");
        let _ = fs::File::create(&archiver_path).expect("Failed to create test archiver file");
    }

    (temp_dir, socket_path, archiver_path)
}

#[test]
fn test_new_with_archiver() {
    let (socket1, socket2) = AlphaSocket::pair().expect("Failed to create socket pair");
    let archiver = Some(socket2);

    let socket_with_archiver = AlphaSocketWithArchiver::new(socket1, archiver);

    assert!(socket_with_archiver.archiver().is_some());
}

#[test]
fn test_pair_with_archiver() {
    let (_temp_dir, _, archiver_path) = setup_temp_paths();

    // Test with archiver path
    let result = AlphaSocketWithArchiver::pair_with_archiver(Some(&archiver_path));

    // Since the actual connection would fail (the archiver isn't a proper socket yet),
    // we should expect both sockets to be created but without archivers
    match result {
        Ok((socket1, socket2)) => {
            assert!(socket1.archiver().is_none());
            assert!(socket2.archiver().is_none());
        }
        Err(e) => panic!("Failed to create pair: {:?}", e),
    }

    // Test without archiver path
    let result: io::Result<(AlphaSocketWithArchiver, AlphaSocketWithArchiver)> =
        AlphaSocketWithArchiver::pair_with_archiver(None::<&Path>);

    match result {
        Ok((socket1, socket2)) => {
            assert!(socket1.archiver().is_none());
            assert!(socket2.archiver().is_none());
        }
        Err(e) => panic!("Failed to create pair without archiver: {:?}", e),
    }
}

#[test]
fn test_read_operation() {
    let (socket1, mut socket2) = AlphaSocket::pair().expect("Failed to create socket pair");

    // Write data to socket2
    let test_data = b"Hello, World!";
    socket2
        .write_all(test_data)
        .expect("Failed to write to socket2");

    // Create socket with archiver using socket1 as the primary
    let mut socket_with_archiver = AlphaSocketWithArchiver::new(socket1, None);

    // Read from socket_with_archiver
    let mut buffer = [0u8; 13];
    let bytes_read = socket_with_archiver
        .read(&mut buffer)
        .expect("Failed to read from socket");

    assert_eq!(bytes_read, test_data.len());
    assert_eq!(&buffer, test_data);
}

#[test]
fn test_write_with_archiver() {
    // Create a socket pair for primary communication
    let (socket1, mut socket2) = AlphaSocket::pair().expect("Failed to create primary socket pair");

    // Create another socket pair for archiving
    let (archiver_socket, mut archiver_reader) =
        AlphaSocket::pair().expect("Failed to create archiver socket pair");

    // Create socket with archiver
    let mut socket_with_archiver = AlphaSocketWithArchiver::new(socket1, Some(archiver_socket));

    // Write data to socket_with_archiver
    let test_data = b"Test message";
    socket_with_archiver
        .write_all(test_data)
        .expect("Failed to write to socket_with_archiver");

    // Read from both the primary and archiver sockets
    let mut primary_buffer = [0u8; 12];
    let mut archiver_buffer = [0u8; 12];

    // Read from primary socket
    let primary_bytes_read = socket2
        .read(&mut primary_buffer)
        .expect("Failed to read from primary socket");

    // Read from archiver socket
    let archiver_bytes_read = archiver_reader
        .read(&mut archiver_buffer)
        .expect("Failed to read from archiver socket");

    // Verify both sockets received the same data
    assert_eq!(primary_bytes_read, test_data.len());
    assert_eq!(&primary_buffer[..primary_bytes_read], test_data);

    assert_eq!(archiver_bytes_read, test_data.len());
    assert_eq!(&archiver_buffer[..archiver_bytes_read], test_data);
}

#[test]
fn test_write_with_archiver_failure() {
    // Create a socket pair for primary communication
    let (socket1, mut socket2) = AlphaSocket::pair().expect("Failed to create primary socket pair");

    // Create another socket pair for archiving, but close the archiver socket to simulate failure
    let (archiver_socket, _) = AlphaSocket::pair().expect("Failed to create archiver socket pair");
    let _ = archiver_socket.shutdown(); // This should make the archiver socket fail on writes

    // Create socket with archiver
    let mut socket_with_archiver = AlphaSocketWithArchiver::new(socket1, Some(archiver_socket));

    // Write data to socket_with_archiver - this should succeed despite archiver failure
    let test_data = b"Test message";
    socket_with_archiver
        .write_all(test_data)
        .expect("Write should succeed despite archiver failure");

    // Read from primary socket to verify data was written
    let mut primary_buffer = [0u8; 12];
    let primary_bytes_read = socket2
        .read(&mut primary_buffer)
        .expect("Failed to read from primary socket");

    // Verify primary socket received the data
    assert_eq!(primary_bytes_read, test_data.len());
    assert_eq!(&primary_buffer[..primary_bytes_read], test_data);
}

#[test]
fn test_read_with_retry() {
    // Create a socket pair
    let (socket1, mut socket2) = AlphaSocket::pair().expect("Failed to create socket pair");

    // Create socket with archiver
    let mut socket_with_archiver = AlphaSocketWithArchiver::new(socket1, None);

    // Write data to socket2
    let test_data = b"Retry test";
    socket2
        .write_all(test_data)
        .expect("Failed to write to socket2");

    // Read with retry from socket_with_archiver
    let mut buffer = [0u8; 10];
    let bytes_read = socket_with_archiver
        .read_with_retry(&mut buffer, 3)
        .expect("Read with retry failed");

    assert_eq!(bytes_read, test_data.len());
    assert_eq!(&buffer, test_data);
}

#[test]
fn test_write_with_retry() {
    // Create a socket pair for primary communication
    let (socket1, mut socket2) = AlphaSocket::pair().expect("Failed to create primary socket pair");

    // Create another socket pair for archiving
    let (archiver_socket, mut archiver_reader) =
        AlphaSocket::pair().expect("Failed to create archiver socket pair");

    // Create socket with archiver
    let mut socket_with_archiver = AlphaSocketWithArchiver::new(socket1, Some(archiver_socket));

    // Write data to socket_with_archiver using write_with_retry
    let test_data = b"Retry write test";
    socket_with_archiver
        .write_with_retry(test_data, 3)
        .expect("Write with retry failed");

    // Read from both the primary and archiver sockets
    let mut primary_buffer = [0u8; 16];
    let mut archiver_buffer = [0u8; 16];

    // Read from primary socket
    let primary_bytes_read = socket2
        .read(&mut primary_buffer)
        .expect("Failed to read from primary socket");

    // Read from archiver socket
    let archiver_bytes_read = archiver_reader
        .read(&mut archiver_buffer)
        .expect("Failed to read from archiver socket");

    // Verify both sockets received the same data
    assert_eq!(primary_bytes_read, test_data.len());
    assert_eq!(&primary_buffer[..primary_bytes_read], test_data);

    assert_eq!(archiver_bytes_read, test_data.len());
    assert_eq!(&archiver_buffer[..archiver_bytes_read], test_data);
}

#[test]
fn test_shutdown() {
    // Create a socket pair for primary communication
    let (socket1, _) = AlphaSocket::pair().expect("Failed to create primary socket pair");

    // Create another socket pair for archiving
    let (archiver_socket, _) = AlphaSocket::pair().expect("Failed to create archiver socket pair");

    // Create socket with archiver
    let socket_with_archiver = AlphaSocketWithArchiver::new(socket1, Some(archiver_socket));

    // Shutdown should succeed
    socket_with_archiver.shutdown().expect("Shutdown failed");

    // After shutdown, further operations should fail
    // We can't reliably test this without adding specific error checking in the AlphaSocket code
}

// Test flush behavior
#[test]
fn test_flush() {
    // Create a socket pair for primary communication
    let (socket1, _) = AlphaSocket::pair().expect("Failed to create primary socket pair");

    // Create another socket pair for archiving
    let (archiver_socket, _) = AlphaSocket::pair().expect("Failed to create archiver socket pair");

    // Create socket with archiver
    let mut socket_with_archiver = AlphaSocketWithArchiver::new(socket1, Some(archiver_socket));

    // Flush should succeed
    socket_with_archiver.flush().expect("Flush failed");
}

// Test connect with buffer size
#[test]
fn test_connect_with_buffer_size() {
    let (_temp_dir, socket_path, archiver_path) = setup_temp_paths();

    // This will fail since these aren't real sockets, but we can check the error handling
    let result = AlphaSocketWithArchiver::connect_with_buffer_size(
        &socket_path,
        Some(&archiver_path),
        Some(1024),
    );

    // Should fail because the socket doesn't exist properly
    assert!(result.is_err());
}

// Test error handling in connect
#[test]
fn test_connect_error_handling() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let nonexistent_path = temp_dir.path().join("nonexistent_socket");
    let archiver_path = temp_dir.path().join("archiver_socket");

    // Create only the archiver file
    let _ = fs::File::create(&archiver_path).expect("Failed to create archiver file");

    // Should fail because primary socket doesn't exist
    let result = AlphaSocketWithArchiver::connect(&nonexistent_path, Some(&archiver_path));
    assert!(result.is_err());
}
