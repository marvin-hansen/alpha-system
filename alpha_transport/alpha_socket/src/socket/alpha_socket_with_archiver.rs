/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::socket::alpha_socket::AlphaSocket;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

/// An AlphaSocket with integrated archiving capabilities
///
/// This extends the standard AlphaSocket with the ability to
/// drop copy all messages to a separate archiver socket with zero
/// latency impact on the primary communication channel.
///
/// Usage:
///
/// 1. Converting an existing AlphaSocket:
/// ```text
/// let socket = AlphaSocket::connect("path/to/socket")?;
/// let archiver = AlphaSocket::connect("path/to/archiver")?;
/// let socket_with_archiver = AlphaSocketWithArchiver::new(socket, Some(archiver));
/// ```
///
/// 2. Direct connection:
/// ```text
/// let socket = AlphaSocketWithArchiver::connect(
///     "path/to/socket",
///     Some("path/to/archiver"),
/// )?;
/// ```
///
/// 3. Creating a pair:
/// ```text
/// let (socket1, socket2) = AlphaSocketWithArchiver::pair_with_archiver(
///     Some("path/to/archiver"),
/// )?;
/// ```
///
pub struct AlphaSocketWithArchiver {
    // The primary socket for bi-directional communication
    inner: AlphaSocket,
    // Optional archiver socket for drop-copy
    archiver: Option<AlphaSocket>,
}

impl AlphaSocketWithArchiver {
    pub fn archiver(&self) -> &Option<AlphaSocket> {
        &self.archiver
    }
}

impl AlphaSocketWithArchiver {
    /// Create a new AlphaSocketWithArchiver from an existing AlphaSocket
    pub fn new(socket: AlphaSocket, archiver: Option<AlphaSocket>) -> Self {
        Self {
            inner: socket,
            archiver,
        }
    }

    /// Connect to a primary socket and optionally an archiver socket
    pub fn connect<P: AsRef<Path>, A: AsRef<Path>>(
        primary_path: P,
        archiver_path: Option<A>,
    ) -> io::Result<Self> {
        // Connect to the primary socket
        let primary_socket = AlphaSocket::connect(primary_path)?;

        // Connect to the archiver socket if provided
        let archiver_socket = if let Some(path) = archiver_path {
            match AlphaSocket::connect(path) {
                Ok(socket) => Some(socket),
                Err(e) => {
                    // Log error but continue without archiver
                    eprintln!("Warning: Failed to connect to archiver socket: {}", e);
                    None
                }
            }
        } else {
            None
        };

        Ok(Self {
            inner: primary_socket,
            archiver: archiver_socket,
        })
    }

    /// Connect with a specific buffer size
    pub fn connect_with_buffer_size<P: AsRef<Path>, A: AsRef<Path>>(
        primary_path: P,
        archiver_path: Option<A>,
        buffer_size: Option<usize>,
    ) -> io::Result<Self> {
        // Connect to the primary socket with specified buffer size
        let primary_socket = AlphaSocket::connect_with_buffer_size(primary_path, buffer_size)?;

        // Connect to the archiver socket if provided
        let archiver_socket = if let Some(path) = archiver_path {
            match AlphaSocket::connect_with_buffer_size(path, buffer_size) {
                Ok(socket) => Some(socket),
                Err(e) => {
                    // Log error but continue without archiver
                    eprintln!("Warning: Failed to connect to archiver socket: {}", e);
                    None
                }
            }
        } else {
            None
        };

        Ok(Self {
            inner: primary_socket,
            archiver: archiver_socket,
        })
    }

    /// Get a reference to the inner socket
    pub fn inner(&self) -> &AlphaSocket {
        &self.inner
    }

    /// Get a mutable reference to the inner socket
    pub fn inner_mut(&mut self) -> &mut AlphaSocket {
        &mut self.inner
    }

    /// Creates a pair of connected AlphaSocketWithArchiver instances with an optional archiver
    pub fn pair_with_archiver<A: AsRef<Path>>(
        archiver_path: Option<A>,
    ) -> io::Result<(Self, Self)> {
        // Create a pair of regular AlphaSockets
        let (socket1, socket2) = AlphaSocket::pair()?;

        // Connect to archiver if path is provided
        let (archiver1, archiver2) = if let Some(path) = archiver_path {
            match (AlphaSocket::connect(&path), AlphaSocket::connect(&path)) {
                (Ok(a1), Ok(a2)) => (Some(a1), Some(a2)),
                _ => {
                    eprintln!("Warning: Failed to connect to archiver socket");
                    (None, None)
                }
            }
        } else {
            (None, None)
        };

        let socket_with_archiver1 = Self {
            inner: socket1,
            archiver: archiver1,
        };

        let socket_with_archiver2 = Self {
            inner: socket2,
            archiver: archiver2,
        };

        Ok((socket_with_archiver1, socket_with_archiver2))
    }

    /// Shutdown the socket and its archiver
    pub fn shutdown(&self) -> io::Result<()> {
        // Shutdown the primary socket
        self.inner.shutdown()?;

        // Shutdown the archiver socket if it exists
        if let Some(ref archiver) = self.archiver {
            archiver.shutdown()?;
        }

        Ok(())
    }
}

// Implement Read trait for AlphaSocketWithArchiver
impl Read for AlphaSocketWithArchiver {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Forward read operation to inner socket
        self.inner.read(buf)
    }
}

// Implement Write trait for AlphaSocketWithArchiver with drop-copy functionality
impl Write for AlphaSocketWithArchiver {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Write to inner socket
        let bytes_written = self.inner.write(buf)?;

        // Don't propagate archiver errors to prevent impacting main communication
        if let Some(archiver) = &mut self.archiver {
            // Try to write to the archiver but ignore errors
            let _ = archiver.write(&buf[..bytes_written]);
        }

        // Return the result from the main socket
        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        // Flush the inner socket
        self.inner.flush()?;

        if let Some(archiver) = &mut self.archiver {
            // Try to flush the archiver but ignore errors
            let _ = archiver.flush();
        }

        Ok(())
    }
}

// Implement high-level methods similar to the original AlphaSocket
impl AlphaSocketWithArchiver {
    pub fn read_with_retry(&mut self, buf: &mut [u8], max_retries: usize) -> io::Result<usize> {
        self.inner.read_with_retry(buf, max_retries)
    }

    pub fn write_with_retry(&mut self, buf: &[u8], max_retries: usize) -> io::Result<()> {
        // First, try to write to the inner socket with retry
        let result = self.inner.write_with_retry(buf, max_retries);

        // If successful; try to write to the archiver
        if let Some(archiver) = &mut self.archiver {
            // Try to write to the archiver but ignore errors
            let _ = archiver.write_with_retry(buf, max_retries);
        }

        // Return the result from the main socket
        result
    }
}

impl Drop for AlphaSocketWithArchiver {
    fn drop(&mut self) {
        // The inner AlphaSocket will be dropped automatically
        // If we have an archiver, it will also be dropped automatically
    }
}
