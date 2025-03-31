/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use alpha_socket::RingBuffer;
use std::io::{Read, Write};
use std::sync::{Arc, Barrier};
use std::thread;
use tempfile::tempdir;

/// Stress test for concurrent read and write operations
#[test]
fn test_ring_buffer_concurrent_access() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("concurrent_buffer");

    // Create a buffer with a known size
    let buffer_size = 1024 * 1024; // 1MB buffer
    let buffer = RingBuffer::new(&path, Some(buffer_size)).unwrap();
    let buffer = Arc::new(std::sync::Mutex::new(buffer));

    // Number of threads and iterations
    let thread_count = 8;
    let iterations = 10_000;

    // Synchronization barrier
    let barrier = Arc::new(Barrier::new(thread_count * 2));

    // Spawn writer threads
    let mut writer_handles = Vec::new();
    for _thread_id in 0..thread_count {
        let buffer_clone = Arc::clone(&buffer);
        let barrier_clone = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier_clone.wait();

            for i in 0..iterations {
                let data = format!("Writer - Iteration {}", i).into_bytes();

                let mut buffer = buffer_clone.lock().unwrap();
                match buffer.write_all(&data) {
                    Ok(_) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // This is expected in high-concurrency scenarios
                        continue;
                    }
                    Err(e) => panic!("Unexpected error: {:?}", e),
                }
            }
        });

        writer_handles.push(handle);
    }

    // Spawn reader threads
    let mut reader_handles = Vec::new();
    for _thread_id in 0..thread_count {
        let buffer_clone = Arc::clone(&buffer);
        let barrier_clone = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier_clone.wait();

            for _ in 0..iterations {
                let mut read_buf = vec![0u8; 1024];

                let mut buffer = buffer_clone.lock().unwrap();
                match buffer.read_exact(&mut read_buf) {
                    Ok(_) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // This is expected in high-concurrency scenarios
                        continue;
                    }
                    Err(e) => panic!("Unexpected error: {:?}", e),
                }
            }
        });

        reader_handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in writer_handles {
        handle.join().unwrap();
    }

    for handle in reader_handles {
        handle.join().unwrap();
    }
}

/// Extensive buffer state validation test
#[test]
fn test_ring_buffer_state_validation() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("state_buffer");

    let mut buffer = RingBuffer::new(&path, Some(1024)).unwrap();

    // Initial state checks
    assert!(buffer.is_empty());
    assert!(!buffer.is_full());
    assert_eq!(buffer.available_data(), 0);
    assert_eq!(buffer.available_space(), 1024);

    // Write partial data
    let partial_data = vec![42u8; 256];
    buffer.write_all(&partial_data).unwrap();

    // Intermediate state checks
    assert!(!buffer.is_empty());
    assert!(!buffer.is_full());
    assert_eq!(buffer.available_data(), 256);
    assert_eq!(buffer.available_space(), 768);

    // Read some data
    let mut read_buf = vec![0u8; 128];
    buffer.read_exact(&mut read_buf).unwrap();

    // Post-read state checks
    assert!(!buffer.is_empty());
    assert!(!buffer.is_full());
    assert_eq!(buffer.available_data(), 128);
    assert_eq!(buffer.available_space(), 896);
}

/// Test extreme edge cases and error conditions
#[test]
fn test_ring_buffer_edge_cases() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("edge_buffer");

    // Attempt to create buffer with invalid size
    {
        let result = RingBuffer::new(&path, Some(100)); // Not a power of 2
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);
    }

    // Create a very small buffer
    let mut buffer = RingBuffer::new(&path, Some(16)).unwrap();

    // Attempt to read from empty buffer
    {
        let mut read_buf = [0u8; 8];
        let read_result = buffer.read(&mut read_buf);
        assert!(read_result.is_err());
        assert_eq!(
            read_result.unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
    }

    // Write data that exactly fills the buffer
    let fill_data = vec![42u8; 16];
    buffer.write_all(&fill_data).unwrap();
    assert!(buffer.is_full());

    // Attempt to write when buffer is full
    {
        let overflow_data = vec![43u8; 8];
        let write_result = buffer.write(&overflow_data);
        assert!(write_result.is_err());
        assert_eq!(
            write_result.unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
    }
}

/// Performance and allocation stress test
#[test]
fn test_ring_buffer_large_allocations() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("large_buffer");

    // Create a large buffer
    let buffer_size = 1024 * 1024 * 16; // 16MB
    let mut buffer = RingBuffer::new(&path, Some(buffer_size)).unwrap();

    // Generate large test data
    let large_data = vec![42u8; buffer_size / 2];

    // Write large chunks multiple times
    for _ in 0..4 {
        buffer.write_all(&large_data).unwrap();

        // Read back the data
        let mut read_buf = vec![0u8; large_data.len()];
        buffer.read_exact(&mut read_buf).unwrap();

        assert_eq!(read_buf, large_data);
    }
}
