/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::RingBuffer;
use std::io;
use std::io::ErrorKind;
use std::sync::atomic::Ordering;

impl RingBuffer {
    /// Read a batch of data from the ring buffer into multiple destination buffers
    /// Returns the total number of bytes read or an error if the buffer is empty
    ///
    /// This method allows for reading multiple chunks at once, reducing the overhead
    /// of multiple atomic operations and function calls.
    ///
    /// # Safety
    /// - All destination buffers must have valid memory
    /// - Maintains atomicity for the entire batch operation
    ///
    /// # Performance
    /// - Uses a single atomic read position update for the entire batch
    /// - Minimizes memory barriers for ultra-low latency
    /// - Optimized pointer arithmetic and memory copies
    #[inline]
    pub fn read_batch(&mut self, bufs: &mut [&mut [u8]]) -> io::Result<usize> {
        if bufs.is_empty() {
            return Ok(0);
        }

        // Load header values with appropriate memory ordering
        let read_pos = self.header().read_pos.load(Ordering::Relaxed);
        let write_pos = self.header().write_pos.load(Ordering::SeqCst); // Use SeqCst for consistent visibility
        let capacity = self.header().capacity.load(Ordering::Relaxed);

        // Quick check if buffer is empty
        if read_pos == write_pos {
            return Err(io::Error::new(ErrorKind::WouldBlock, "Buffer is empty"));
        }

        // Calculate available data
        let available = write_pos - read_pos;
        if available == 0 {
            return Err(io::Error::new(ErrorKind::WouldBlock, "Buffer is empty"));
        }

        // Bit mask for power of 2 capacity
        let mask = capacity - 1;

        // Start with the physical read position
        let mut physical_pos = (read_pos & mask) as usize;
        let mut total_read = 0;

        // Process each buffer in the batch
        for buf in bufs {
            // Don't exceed available data
            let read_size = std::cmp::min(buf.len() as u64, available - total_read as u64) as usize;
            if read_size == 0 {
                break; // No more data available
            }

            // Optimized copy with wrap-around handling
            if physical_pos + read_size <= capacity as usize {
                // No wrap-around, single copy
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        self.buffer_ptr().add(physical_pos),
                        buf.as_mut_ptr(),
                        read_size,
                    );
                }

                // Update physical position
                physical_pos = (physical_pos + read_size) & (capacity as usize - 1);
            } else {
                // Handle wrap-around with two copies
                let first_chunk = capacity as usize - physical_pos;
                unsafe {
                    // Copy first chunk (up to end of buffer)
                    std::ptr::copy_nonoverlapping(
                        self.buffer_ptr().add(physical_pos),
                        buf.as_mut_ptr(),
                        first_chunk,
                    );

                    // Copy second chunk (from beginning of buffer)
                    std::ptr::copy_nonoverlapping(
                        self.buffer_ptr(),
                        buf.as_mut_ptr().add(first_chunk),
                        read_size - first_chunk,
                    );
                }

                // Update physical position
                physical_pos = (read_size - first_chunk) & (capacity as usize - 1);
            }

            total_read += read_size;
            if (total_read as u64) >= available {
                break; // We've consumed all available data
            }
        }

        if total_read > 0 {
            // Only update read position once for the entire batch
            // Use SeqCst for proper visibility to writers
            self.header()
                .read_pos
                .store(read_pos + total_read as u64, Ordering::SeqCst);
        }

        Ok(total_read)
    }
}
