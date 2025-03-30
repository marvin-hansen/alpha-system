use alpha_socket::RingBuffer;
use std::io::{Read, Write};
use tempfile::tempdir;

#[test]
fn test_ring_buffer_basic_operations() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_buffer");

    // Test creating a new buffer
    let mut buffer = RingBuffer::new(&path, Some(1024)).unwrap();

    // Test writing data
    let test_data = b"Hello, World!";
    assert_eq!(buffer.write(test_data).unwrap(), test_data.len());

    // Test reading data
    let mut read_buf = vec![0u8; test_data.len()];
    assert_eq!(buffer.read(&mut read_buf).unwrap(), test_data.len());
    assert_eq!(&read_buf, test_data);
}

#[test]
fn test_ring_buffer_wrap_around() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_buffer");

    // Create a small buffer to force wrap-around
    let mut buffer = RingBuffer::new(&path, Some(256)).unwrap();

    // Write data that will cause wrap-around
    let data1 = b"First message that will wrap around";
    let data2 = b"Second message";

    assert_eq!(buffer.write(data1).unwrap(), data1.len());
    assert_eq!(buffer.write(data2).unwrap(), data2.len());

    // Read back the data
    let mut read_buf1 = vec![0u8; data1.len()];
    let mut read_buf2 = vec![0u8; data2.len()];

    assert_eq!(buffer.read(&mut read_buf1).unwrap(), data1.len());
    assert_eq!(buffer.read(&mut read_buf2).unwrap(), data2.len());

    assert_eq!(&read_buf1, data1);
    assert_eq!(&read_buf2, data2);
}

#[test]
fn test_ring_buffer_full() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_buffer");

    // Create a small buffer with a known size
    let buffer_size = 64; // 64 bytes
    let mut buffer = RingBuffer::new(&path, Some(buffer_size)).unwrap();

    // Create data that exceeds the buffer size
    let large_data = vec![42u8; buffer_size + 10]; // 10 bytes more than buffer_size

    // Trying to write more data than the buffer can hold should fail
    let result = buffer.write(&large_data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::WouldBlock);

    // But writing a smaller amount should work
    let small_data = vec![42u8; buffer_size / 2]; // Half the buffer size
    assert!(buffer.write(&small_data).is_ok());
}

#[test]
fn test_ring_buffer_invalid_size() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_buffer");

    // Try to create a buffer with non-power-of-2 size
    let result = RingBuffer::new(&path, Some(100));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);
}
