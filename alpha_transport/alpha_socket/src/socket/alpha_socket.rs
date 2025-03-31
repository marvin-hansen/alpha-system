use crate::socket::{DEFAULT_BUFFER_SIZE, DEFAULT_TIMEOUT_MS};
use crate::{RingBuffer, RingBufferHeader};
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::time::{Duration, Instant};
use std::{io, thread};

/// A Unix domain socket stream implementation using AlphaSocket technology
///
/// This stream can be used to read and write data to a Unix domain socket
pub struct AlphaSocket {
    read_buffer: RingBuffer,
    write_buffer: RingBuffer,
    _keep_alive: File, // Keep socket file open for the lifetime of the stream
}

impl AlphaSocket {
    pub fn new(read_buffer: RingBuffer, write_buffer: RingBuffer, keep_alive: File) -> Self {
        Self {
            read_buffer,
            write_buffer,
            _keep_alive: keep_alive,
        }
    }
}
impl Read for AlphaSocket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read_buffer.read(buf)
    }
}

impl Write for AlphaSocket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write_buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl AlphaSocket {
    pub fn read_with_retry(&mut self, buf: &mut [u8], max_retries: usize) -> io::Result<usize> {
        let mut attempt = 0;
        let sleep_time = Duration::from_micros(500);

        loop {
            match self.read(buf) {
                Ok(n) => {
                    return Ok(n);
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    attempt += 1;
                    if attempt >= max_retries {
                        return Err(e);
                    }

                    // Yield to allow other threads to progress
                    std::thread::yield_now();
                    std::thread::sleep(sleep_time);

                    continue;
                }
                Err(e) => {
                    println!("read_with_retry - Read error: {:?}", e);
                    return Err(e);
                }
            }
        }
    }

    pub fn write_with_retry(&mut self, buf: &[u8], max_retries: usize) -> io::Result<()> {
        let mut attempt = 0;
        let sleep_time = Duration::from_micros(500);

        loop {
            match self.write(buf) {
                Ok(n) => {
                    if n == buf.len() {
                        // Yield to help synchronize buffer state
                        std::thread::yield_now();
                        return self.flush();
                    } else {
                        return Err(io::Error::new(ErrorKind::Other, "Partial write"));
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    attempt += 1;
                    if attempt >= max_retries {
                        return Err(e);
                    }

                    // Yield to allow other threads to progress
                    std::thread::yield_now();
                    std::thread::sleep(sleep_time);

                    continue;
                }
                Err(e) => {
                    println!("write_with_retry - Write error: {:?}", e);
                    return Err(e);
                }
            }
        }
    }

    /// Connect to an existing socket
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Self::connect_with_buffer_size(path, None)
    }

    /// Connect to an existing socket with a specific buffer size
    pub fn connect_with_buffer_size<P: AsRef<Path>>(
        path: P,
        buffer_size: Option<usize>,
    ) -> io::Result<Self> {
        let socket_path = path.as_ref();

        // Check if the socket exists
        if !socket_path.exists() {
            return Err(io::Error::new(ErrorKind::NotFound, "Socket does not exist"));
        }

        // Server's write buffer is client's read buffer
        let client_read_path = socket_path.with_extension("server-write");

        // Client's write buffer
        let client_write_path = socket_path.with_extension("client-write");

        // Wait for server's write buffer
        let start = Instant::now();
        while !client_read_path.exists() {
            if start.elapsed() > Duration::from_millis(DEFAULT_TIMEOUT_MS) {
                return Err(io::Error::new(
                    ErrorKind::TimedOut,
                    "Connection timeout waiting for server write buffer",
                ));
            }
            thread::sleep(Duration::from_millis(50));
        }

        // Create client write buffer file
        let client_write_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o666)
            .truncate(true)
            .open(&client_write_path)?;

        // Ensure buffer file size
        let buffer_size_bytes = buffer_size.unwrap_or(64 * 1024);
        let total_size = size_of::<RingBufferHeader>() + buffer_size_bytes;
        client_write_file.set_len(total_size as u64)?;
        client_write_file.sync_all()?;

        // Open server write buffer (client read buffer)
        let read_buffer = RingBuffer::open(&client_read_path, buffer_size)?;

        // Create client write buffer
        let write_buffer = RingBuffer::new(&client_write_path, buffer_size)?;

        // Keep socket file open
        let keep_alive = File::open(socket_path)?;

        Ok(Self {
            read_buffer,
            write_buffer,
            _keep_alive: keep_alive,
        })
    }

    /// Creates a pair of connected AlphaStream instances.
    ///
    /// Returns two `AlphaStream`s which are connected to each other.
    ///
    /// # Examples
    ///
    /// ```
    /// use alpha_socket::AlphaSocket;
    ///
    /// let (stream1, stream2) = AlphaSocket::pair().expect("Failed to create stream pair");
    /// ```
    pub fn pair() -> io::Result<(Self, Self)> {
        // Create a temporary directory
        let temp_dir = tempfile::tempdir()?;

        // Single shared socket path
        let shared_socket_path = temp_dir.path().join("shared_socket");

        // Create a single shared socket file
        let shared_socket_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o600)
            .truncate(true)
            .open(&shared_socket_path)?;

        // Set buffer size
        let buffer_size = DEFAULT_BUFFER_SIZE;

        // Create buffers for communication
        let stream1_write_path = temp_dir.path().join("stream1_write");
        let stream2_write_path = temp_dir.path().join("stream2_write");

        // Create write buffers
        let stream1_write_buffer = RingBuffer::new(&stream1_write_path, Some(buffer_size))?;
        let stream2_write_buffer = RingBuffer::new(&stream2_write_path, Some(buffer_size))?;

        // Create streams with linked buffers
        let stream1 = Self {
            // stream1's read buffer is stream2's write buffer
            read_buffer: RingBuffer::open(&stream2_write_path, Some(buffer_size))?,
            write_buffer: stream1_write_buffer,
            _keep_alive: shared_socket_file.try_clone()?,
        };

        let stream2 = Self {
            // stream2's read buffer is stream1's write buffer
            read_buffer: RingBuffer::open(&stream1_write_path, Some(buffer_size))?,
            write_buffer: stream2_write_buffer,
            _keep_alive: shared_socket_file,
        };

        Ok((stream1, stream2))
    }

    /// Creates a new pair of streams for communication
    ///
    /// # Errors
    ///
    /// This function will return an error if the underlying file operations fail.
    ///
    pub fn shutdown(&self) -> io::Result<()> {
        // Close the read and write buffers
        self.read_buffer.close()?;
        self.write_buffer.close()?;
        Ok(())
    }
}

impl Drop for AlphaSocket {
    fn drop(&mut self) {
        // Clean up client write buffer file
        let write_path = self.write_buffer.path();
        let _ = std::fs::remove_file(write_path);
    }
}
