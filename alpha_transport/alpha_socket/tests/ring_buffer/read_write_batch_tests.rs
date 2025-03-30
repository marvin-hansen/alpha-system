/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use alpha_socket::RingBuffer;
use std::io::ErrorKind;
use tempfile::tempdir;

#[test]
fn test_basic_batch_operations() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("batch_test_buffer");

    // Create a new buffer
    let mut buffer = RingBuffer::new(&path, Some(1024)).unwrap();

    // Prepare data to write in batches
    let data1 = b"First batch message";
    let data2 = b"Second batch message";
    let data3 = b"Third batch message";

    // Write data in a single batch
    let write_bufs = [data1.as_slice(), data2.as_slice(), data3.as_slice()];
    let total_size = data1.len() + data2.len() + data3.len();

    let written = buffer.write_batch(&write_bufs).unwrap();
    assert_eq!(written, total_size, "Should write all data in the batch");

    // Read data in a single batch
    let mut read_buf1 = vec![0u8; data1.len()];
    let mut read_buf2 = vec![0u8; data2.len()];
    let mut read_buf3 = vec![0u8; data3.len()];

    let mut read_bufs = [
        read_buf1.as_mut_slice(),
        read_buf2.as_mut_slice(),
        read_buf3.as_mut_slice(),
    ];

    let read = buffer.read_batch(&mut read_bufs).unwrap();
    assert_eq!(read, total_size, "Should read all data from the batch");

    // Verify the data matches
    assert_eq!(read_buf1, data1, "First buffer should match");
    assert_eq!(read_buf2, data2, "Second buffer should match");
    assert_eq!(read_buf3, data3, "Third buffer should match");
}

#[test]
fn test_mixed_batch_sizes() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("mixed_batch_buffer");

    // Create a new buffer
    let mut buffer = RingBuffer::new(&path, Some(1024)).unwrap();

    // Prepare data of varying sizes
    let small_data = b"Small";
    let medium_data = b"Medium sized message";
    let large_data = vec![b'X'; 256]; // 256 bytes of 'X'

    // Write data in a batch
    let write_bufs = [
        small_data.as_slice(),
        medium_data.as_slice(),
        large_data.as_slice(),
    ];

    let total_size = small_data.len() + medium_data.len() + large_data.len();
    let written = buffer.write_batch(&write_bufs).unwrap();
    assert_eq!(
        written, total_size,
        "Should write all data regardless of varying sizes"
    );

    // Read data in a batch
    let mut read_buf1 = vec![0u8; small_data.len()];
    let mut read_buf2 = vec![0u8; medium_data.len()];
    let mut read_buf3 = vec![0u8; large_data.len()];

    let mut read_bufs = [
        read_buf1.as_mut_slice(),
        read_buf2.as_mut_slice(),
        read_buf3.as_mut_slice(),
    ];

    let read = buffer.read_batch(&mut read_bufs).unwrap();
    assert_eq!(read, total_size, "Should read all data from the batch");

    // Verify the data matches
    assert_eq!(read_buf1, small_data, "Small buffer should match");
    assert_eq!(read_buf2, medium_data, "Medium buffer should match");
    assert_eq!(read_buf3, large_data, "Large buffer should match");
}

#[test]
fn test_batch_wrap_around() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("wrap_around_buffer");

    // Create a smaller buffer to force wrap-around
    let mut buffer = RingBuffer::new(&path, Some(128)).unwrap();

    // Write and read some data to advance the positions within the buffer
    let initial_data = vec![b'I'; 100]; // 100 bytes, pushing us near the end of the buffer
    let write_bufs_initial = [initial_data.as_slice()];
    buffer.write_batch(&write_bufs_initial).unwrap();

    // Read the initial data to advance read position
    let mut read_initial = vec![0u8; 100];
    let mut read_bufs_initial = [read_initial.as_mut_slice()];
    buffer.read_batch(&mut read_bufs_initial).unwrap();

    // Now the next write should wrap around
    // Prepare data for the wrap-around test
    let data1 = b"This will start near the end";
    let data2 = b"This will wrap around to the beginning";

    let write_bufs = [data1.as_slice(), data2.as_slice()];
    let total_size = data1.len() + data2.len();

    let written = buffer.write_batch(&write_bufs).unwrap();
    assert_eq!(
        written, total_size,
        "Should write all data across the wrap-around"
    );

    // Read the data back
    let mut read_buf1 = vec![0u8; data1.len()];
    let mut read_buf2 = vec![0u8; data2.len()];

    let mut read_bufs = [read_buf1.as_mut_slice(), read_buf2.as_mut_slice()];

    let read = buffer.read_batch(&mut read_bufs).unwrap();
    assert_eq!(
        read, total_size,
        "Should read all data across the wrap-around"
    );

    // Verify data integrity
    assert_eq!(
        read_buf1, data1,
        "First buffer should match even with wrap-around"
    );
    assert_eq!(
        read_buf2, data2,
        "Second buffer should match even with wrap-around"
    );
}

#[test]
fn test_batch_edge_cases() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("edge_case_buffer");

    // Create a buffer with a known size
    let buffer_size = 256;
    let mut buffer = RingBuffer::new(&path, Some(buffer_size)).unwrap();

    // Test 1: Empty batch
    let empty_bufs: [&[u8]; 0] = [];
    assert_eq!(
        buffer.write_batch(&empty_bufs).unwrap(),
        0,
        "Writing empty batch should return 0"
    );

    let mut empty_read_bufs: [&mut [u8]; 0] = [];
    assert_eq!(
        buffer.read_batch(&mut empty_read_bufs).unwrap(),
        0,
        "Reading empty batch should return 0"
    );

    // Test 2: Batch with empty buffers
    let data = b"Real data";
    let empty: &[u8] = &[]; // Declare empty as a slice
    let write_bufs = [empty, data, empty];

    let written = buffer.write_batch(&write_bufs).unwrap();
    assert_eq!(written, data.len(), "Should only count non-empty buffers");

    // Test 3: Buffer full condition
    // Fill the buffer first
    let large_data = vec![b'X'; buffer_size - 20]; // Leave some room
    buffer.write_batch(&[large_data.as_slice()]).unwrap();

    // Now try to write more than available space
    let overflow_data = [
        b"This should fail".as_slice(),
        b"Because buffer is almost full".as_slice(),
    ];
    let result = buffer.write_batch(&overflow_data);
    assert!(result.is_err(), "Should fail when buffer is full");
    assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::WouldBlock,
        "Should return WouldBlock error"
    );
}

#[test]
fn test_sequential_batch_operations() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("sequential_buffer");

    // Create a buffer
    let mut buffer = RingBuffer::new(&path, Some(512)).unwrap();

    // Test multiple sequential write and read operations
    for i in 0..5 {
        // Generate unique data for each iteration
        let data1 = format!("Batch {} - Message 1", i).into_bytes();
        let data2 = format!("Batch {} - Message 2", i).into_bytes();

        // Write the batch
        let write_bufs = [data1.as_slice(), data2.as_slice()];
        let total_size = data1.len() + data2.len();

        let written = buffer.write_batch(&write_bufs).unwrap();
        assert_eq!(written, total_size, "Should write complete batch {}", i);

        // Read the batch back
        let mut read_buf1 = vec![0u8; data1.len()];
        let mut read_buf2 = vec![0u8; data2.len()];

        let mut read_bufs = [read_buf1.as_mut_slice(), read_buf2.as_mut_slice()];

        let read = buffer.read_batch(&mut read_bufs).unwrap();
        assert_eq!(read, total_size, "Should read complete batch {}", i);

        // Verify data integrity
        assert_eq!(read_buf1, data1, "First buffer should match in batch {}", i);
        assert_eq!(
            read_buf2, data2,
            "Second buffer should match in batch {}",
            i
        );
    }
}

#[test]
fn test_partial_batch_reads() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("partial_read_buffer");

    // Create a buffer
    let mut buffer = RingBuffer::new(&path, Some(512)).unwrap();

    // Prepare data
    let data1 = b"First message in batch";
    let data2 = b"Second message in batch";
    let data3 = b"Third message in batch";

    // Write all data in one batch
    let write_bufs = [data1.as_slice(), data2.as_slice(), data3.as_slice()];
    buffer.write_batch(&write_bufs).unwrap();

    // Read only part of the data first
    let mut read_buf1 = vec![0u8; data1.len()];
    let mut read_bufs1 = [read_buf1.as_mut_slice()];

    let read1 = buffer.read_batch(&mut read_bufs1).unwrap();
    assert_eq!(read1, data1.len(), "Should read first buffer");
    assert_eq!(read_buf1, data1, "First buffer should match");

    // Read the rest in a separate operation
    let mut read_buf2 = vec![0u8; data2.len()];
    let mut read_buf3 = vec![0u8; data3.len()];

    let mut read_bufs2 = [read_buf2.as_mut_slice(), read_buf3.as_mut_slice()];

    let read2 = buffer.read_batch(&mut read_bufs2).unwrap();
    assert_eq!(
        read2,
        data2.len() + data3.len(),
        "Should read remaining buffers"
    );
    assert_eq!(read_buf2, data2, "Second buffer should match");
    assert_eq!(read_buf3, data3, "Third buffer should match");
}

#[test]
fn test_read_batch_empty_buffer() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("empty_read_buffer");

    // Create a buffer
    let mut buffer = RingBuffer::new(&path, Some(256)).unwrap();

    // Try to read from an empty buffer
    let mut read_buf = vec![0u8; 10];
    let mut read_bufs = [read_buf.as_mut_slice()];

    let result = buffer.read_batch(&mut read_bufs);
    assert!(result.is_err(), "Reading from empty buffer should fail");
    assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::WouldBlock,
        "Should return WouldBlock error"
    );
}

#[test]
fn test_batch_with_smaller_read_buffers() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("smaller_read_buffer");

    // Create a buffer
    let mut buffer = RingBuffer::new(&path, Some(256)).unwrap();

    // Prepare data
    let data = b"This is a larger message that won't fit in the read buffer";

    // Write the data
    buffer.write_batch(&[data.as_slice()]).unwrap();

    // Read with a smaller buffer
    let mut small_buf = vec![0u8; 20]; // Smaller than the written data
    let mut read_bufs = [small_buf.as_mut_slice()];

    let read = buffer.read_batch(&mut read_bufs).unwrap();
    assert_eq!(
        read, 20,
        "Should read only what fits in the provided buffer"
    );
    assert_eq!(
        small_buf,
        &data[0..20],
        "Should match the first part of the data"
    );

    // Read the rest
    let mut rest_buf = vec![0u8; data.len() - 20];
    let mut rest_bufs = [rest_buf.as_mut_slice()];

    let read_rest = buffer.read_batch(&mut rest_bufs).unwrap();
    assert_eq!(
        read_rest,
        data.len() - 20,
        "Should read the remainder of the data"
    );
    assert_eq!(rest_buf, &data[20..], "Should match the rest of the data");
}
