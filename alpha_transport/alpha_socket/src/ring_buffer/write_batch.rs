/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::RingBuffer;
use std::io;
use std::io::ErrorKind;
use std::sync::atomic::Ordering;

impl RingBuffer {
    /// Write a batch of data to the ring buffer from multiple source buffers
    /// Returns the total number of bytes written or an error if the buffer is full
    ///
    /// This method allows for writing multiple chunks at once, reducing the overhead
    /// of multiple atomic operations and function calls.
    ///
    /// # Safety
    /// - All source buffers must have valid memory
    /// - Maintains atomicity for the entire batch operation
    ///
    /// # Performance
    /// - Uses a single atomic write position update for the entire batch
    /// - Minimizes memory barriers for ultra-low latency
    /// - Optimized pointer arithmetic and memory copies
    #[inline]
    pub fn write_batch(&mut self, bufs: &[&[u8]]) -> io::Result<usize> {
        if bufs.is_empty() {
            return Ok(0);
        }

        // Calculate total bytes to write
        let mut total_size: usize = 0;
        for buf in bufs {
            total_size += buf.len();
        }
        if total_size == 0 {
            return Ok(0);
        }

        // Load header values with appropriate memory ordering
        let read_pos = self.header().read_pos.load(Ordering::SeqCst); // SeqCst for consistent visibility
        let write_pos = self.header().write_pos.load(Ordering::Relaxed);
        let capacity = self.header().capacity.load(Ordering::Relaxed);

        // Calculate available space
        let available_space = capacity - (write_pos - read_pos);

        // Check if entire batch can be written
        if total_size as u64 > available_space {
            return Err(io::Error::new(ErrorKind::WouldBlock, "Buffer is full"));
        }

        // Bit mask for power of 2 capacity (for wrap-around)
        let mask = capacity - 1;

        // Start with the physical write position
        let mut physical_pos = (write_pos & mask) as usize;
        let mut total_written = 0;

        // Process each buffer in the batch
        for buf in bufs {
            let write_size = buf.len();
            if write_size == 0 {
                continue;
            }

            // Handle buffer wrap-around when writing
            if physical_pos + write_size <= capacity as usize {
                // No wrap-around, single copy
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        buf.as_ptr(),
                        self.buffer_ptr_mut().add(physical_pos),
                        write_size,
                    );
                }

                // Update physical position
                physical_pos = (physical_pos + write_size) & (capacity as usize - 1);
            } else {
                // Handle wrap-around with two copies
                let first_chunk = capacity as usize - physical_pos;
                unsafe {
                    // Copy first chunk (up to end of buffer)
                    std::ptr::copy_nonoverlapping(
                        buf.as_ptr(),
                        self.buffer_ptr_mut().add(physical_pos),
                        first_chunk,
                    );

                    // Copy second chunk (from beginning of buffer)
                    std::ptr::copy_nonoverlapping(
                        buf.as_ptr().add(first_chunk),
                        self.buffer_ptr_mut(),
                        write_size - first_chunk,
                    );
                }

                // Update physical position
                physical_pos = (write_size - first_chunk) & (capacity as usize - 1);
            }

            total_written += write_size;
        }

        // Ensure all memory writes are visible before updating write position
        std::sync::atomic::fence(Ordering::Release);

        // Update write position with a single atomic operation for the entire batch
        self.header()
            .write_pos
            .store(write_pos + total_written as u64, Ordering::SeqCst);

        Ok(total_written)
    }
}
