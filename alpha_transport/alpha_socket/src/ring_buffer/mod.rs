mod getters;
mod read;
mod write;

use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::io::{self, ErrorKind};
use std::mem::size_of;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

// Constants for buffer configuration
const DEFAULT_BUFFER_SIZE: usize = 64 * 1024; // 64KB default buffer size (power of 2)

/// Ring buffer header with cache line padding to prevent false sharing
#[repr(C, align(64))]
pub struct RingBufferHeader {
    // Producer fields (64-byte cache line)
    write_pos: AtomicU64,
    _padding1: [u8; 56], // Padding to fill cache line (64 - 8 bytes)

    // Consumer fields (64-byte cache line)
    read_pos: AtomicU64,
    _padding2: [u8; 56], // Padding to fill cache line (64 - 8 bytes)

    // Shared fields (64-byte cache line)
    capacity: AtomicU64,
    _padding3: [u8; 56], // Padding to fill cache line (64 - 8 bytes)
}

/// Constants for ring buffer configuration
pub(crate) const HEADER_SIZE: usize = size_of::<RingBufferHeader>();

/// A high-performance, lock-free ring buffer implementation
/// Designed for ultra-low latency IPC with single-digit nanosecond messaging
#[derive(Debug)]
pub struct RingBuffer {
    mmap: MmapMut,
    path: PathBuf,
    delete_on_drop: bool,
}

impl RingBuffer {
    /// Creates a new ring buffer with the specified size (must be a power of 2)
    pub fn new<P: AsRef<Path>>(path: P, buffer_size: Option<usize>) -> io::Result<Self> {
        Self::new_internal(path, buffer_size, true)
    }

    /// Opens an existing ring buffer
    pub fn open<P: AsRef<Path>>(path: P, buffer_size: Option<usize>) -> io::Result<Self> {
        Self::new_internal(path, buffer_size, false)
    }

    /// Closes the ring buffer to its initial state, overwriting all data in the buffer
    pub fn close(&self) -> io::Result<()> {
        // Delete the file
        let _ = std::fs::remove_file(&self.path);

        Ok(())
    }

    /// Internal implementation for creating or opening a ring buffer
    fn new_internal<P: AsRef<Path>>(
        path: P,
        buffer_size: Option<usize>,
        delete_on_drop: bool,
    ) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let buffer_size = buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE);

        // Ensure buffer size is a power of 2 for efficient wrapping
        if !buffer_size.is_power_of_two() {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "Buffer size must be a power of 2",
            ));
        }

        // Open file with appropriate options
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?;

        // Set file size to header size + buffer size if needed
        let total_size = HEADER_SIZE + buffer_size;
        let metadata = file.metadata()?;

        // Only truncate and initialize if the file is empty or too small
        if metadata.len() < total_size as u64 {
            file.set_len(total_size as u64)?;

            // Memory map the file
            let mut mmap = unsafe { MmapOptions::new().map_mut(&file)? };

            // Initialize header fields
            let header = unsafe { &mut *(mmap.as_mut_ptr() as *mut RingBufferHeader) };
            header.read_pos.store(0, Ordering::SeqCst);
            header.write_pos.store(0, Ordering::SeqCst);
            header.capacity.store(buffer_size as u64, Ordering::SeqCst);

            // Zero out the buffer for clean state
            for i in HEADER_SIZE..total_size {
                mmap[i] = 0;
            }

            Ok(Self {
                mmap,
                path,
                delete_on_drop,
            })
        } else {
            // Memory map the existing file
            let mmap = unsafe { MmapOptions::new().map_mut(&file)? };

            Ok(Self {
                mmap,
                path,
                delete_on_drop,
            })
        }
    }
}

impl Drop for RingBuffer {
    fn drop(&mut self) {
        // Flush to disk before dropping
        let _ = self.mmap.flush();

        // Delete the file if configured to do so
        if self.delete_on_drop {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}
