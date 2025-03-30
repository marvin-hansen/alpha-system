use crate::RingBuffer;
use std::io;
use std::io::{ErrorKind, Write};
use std::sync::atomic::Ordering;

impl Write for RingBuffer {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Load header values with appropriate memory ordering
        let read_pos = self.header().read_pos.load(Ordering::SeqCst); // Use SeqCst for consistent visibility
        let write_pos = self.header().write_pos.load(Ordering::Relaxed);
        let capacity = self.header().capacity.load(Ordering::Relaxed);

        // Calculate available space
        let available_space = capacity - (write_pos - read_pos);

        // Check if the entire buffer can be written
        if buf.len() as u64 > available_space {
            return Err(io::Error::new(ErrorKind::WouldBlock, "Buffer is full"));
        }

        // Calculate write size
        let write_size = buf.len();

        // Calculate physical position in the buffer (mask for power of 2 wrap-around)
        let mask = capacity - 1;
        let physical_pos = (write_pos & mask) as usize;

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
        }

        // Ensure all memory writes are visible before updating write position
        std::sync::atomic::fence(Ordering::Release);

        // Update write position with release ordering for visibility to reader
        self.header()
            .write_pos
            .store(write_pos + write_size as u64, Ordering::SeqCst);

        Ok(write_size)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        // For memory-mapped files, we can use msync to ensure data is flushed to disk
        // But for performance, we'll just issue a memory fence
        std::sync::atomic::fence(Ordering::SeqCst);
        Ok(())
    }
}
