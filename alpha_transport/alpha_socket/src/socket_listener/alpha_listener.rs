use crate::socket_stream::{DEFAULT_BUFFER_SIZE, DEFAULT_TIMEOUT_MS};
use crate::{AlphaStream, RingBuffer, RingBufferHeader};
use std::fs::{File, OpenOptions};
use std::io::ErrorKind;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::{io, thread};

/// A Unix domain socket listener implementation using AlphaSocket technology
///
/// This listener can accept connections and turn them into stream objects
pub struct AlphaListener {
    path: PathBuf,
    buffer_size: Option<usize>,
    _socket_file: File, // Keep socket file open for the lifetime of the listener
    server_write_path: PathBuf,
    client_write_path: PathBuf,
}

impl AlphaListener {
    /// Bind to a specific socket path
    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();

        // Remove any existing socket file
        let _ = std::fs::remove_file(&path);

        // Create server write buffer path
        let server_write_path = path.with_extension("server-write");
        let _ = std::fs::remove_file(&server_write_path);

        let client_write_path = path.with_extension("client-write");
        let _ = std::fs::remove_file(&client_write_path);

        // Create socket file to mark as bound
        let socket_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o666)
            .truncate(true)
            .open(&path)?;

        // Default buffer size

        // Create the server-write buffer file
        let server_write_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o666)
            .truncate(true)
            .open(&server_write_path)?;

        // Ensure the file has the right size
        let total_size = size_of::<RingBufferHeader>() + DEFAULT_BUFFER_SIZE;
        server_write_file.set_len(total_size as u64)?;

        // Create the RingBuffer
        let buffer_size = Some(DEFAULT_BUFFER_SIZE);
        let _server_write_buffer = RingBuffer::new(&server_write_path, buffer_size)?;

        // Sync files
        socket_file.sync_all()?;
        server_write_file.sync_all()?;

        // Create an empty client write buffer file to signal readiness
        let client_write_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o666)
            .truncate(true)
            .open(&client_write_path)?;
        client_write_file.sync_all()?;

        Ok(Self {
            path,
            buffer_size,
            _socket_file: socket_file,
            server_write_path,
            client_write_path,
        })
    }

    /// Accept a single connection
    pub fn accept(&self) -> io::Result<AlphaStream> {
        // Wait for client to create its write buffer
        let client_write_path = &self.client_write_path;
        let start = Instant::now();
        while !client_write_path.exists() {
            if start.elapsed() > Duration::from_millis(DEFAULT_TIMEOUT_MS) {
                return Err(io::Error::new(
                    ErrorKind::TimedOut,
                    "Connection timeout waiting for client write buffer",
                ));
            }
            thread::sleep(Duration::from_millis(50));
        }

        // Open client write buffer (server read buffer)
        let read_buffer = RingBuffer::open(client_write_path, self.buffer_size)?;

        // Create server write buffer (client read buffer)
        let write_buffer = RingBuffer::open(&self.server_write_path, self.buffer_size)?;

        // Keep socket file open
        let keep_alive = File::open(&self.path)?;

        Ok(AlphaStream::new(read_buffer, write_buffer, keep_alive))
    }

    /// Creates a new independently owned handle to the underlying socket.
    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self {
            path: self.path.clone(),
            buffer_size: self.buffer_size,
            _socket_file: self._socket_file.try_clone()?,
            server_write_path: self.server_write_path.clone(),
            client_write_path: self.client_write_path.clone(),
        })
    }

    /// Returns the local socket address of this listener.
    pub fn local_addr(&self) -> io::Result<PathBuf> {
        Ok(self.path.clone())
    }

    /// Returns an iterator over incoming connections.
    pub fn incoming(&self) -> Incoming {
        Incoming {
            listener: self.try_clone().unwrap(),
        }
    }

    /// Explicitly shut down the listener and clean up resources
    pub fn shutdown(&mut self) -> io::Result<()> {
        // Clean up socket and buffer files
        std::fs::remove_file(&self.path)?;
        std::fs::remove_file(&self.server_write_path)?;
        std::fs::remove_file(&self.client_write_path)?;
        Ok(())
    }
}

impl Drop for AlphaListener {
    fn drop(&mut self) {
        // Clean up socket and buffer files
        let _ = std::fs::remove_file(&self.path);
        let _ = std::fs::remove_file(&self.server_write_path);
        let _ = std::fs::remove_file(&self.client_write_path);
    }
}

/// Iterator over incoming connections.
pub struct Incoming {
    listener: AlphaListener,
}

impl Iterator for Incoming {
    type Item = io::Result<AlphaStream>;

    fn next(&mut self) -> Option<io::Result<AlphaStream>> {
        Some(self.listener.accept())
    }
}
