/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::RingBuffer;
use std::io;
use std::io::{ErrorKind, Read};
use std::sync::atomic::Ordering;

impl Read for RingBuffer {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Load header values with appropriate memory ordering
        let read_pos = self.header().read_pos.load(Ordering::Relaxed);
        let write_pos = self.header().write_pos.load(Ordering::SeqCst); // Use SeqCst for consistent visibility
        let capacity = self.header().capacity.load(Ordering::Relaxed);

        // Quick check if buffer is empty
        if read_pos == write_pos {
            return Err(io::Error::new(ErrorKind::WouldBlock, "Buffer is empty"));
        }

        // Calculate available data and read size
        let available = write_pos - read_pos;
        let read_size = std::cmp::min(buf.len() as u64, available) as usize;

        if read_size == 0 {
            return Err(io::Error::new(ErrorKind::WouldBlock, "Buffer is empty"));
        }

        // Calculate physical position in the buffer
        let mask = capacity - 1; // Bit mask for power of 2 capacity
        let physical_pos = (read_pos & mask) as usize;

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
        }

        // Update read position with release ordering for visibility to writer
        self.header()
            .read_pos
            .store(read_pos + read_size as u64, Ordering::SeqCst);

        Ok(read_size)
    }
}
