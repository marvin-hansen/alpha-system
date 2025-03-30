use crate::ring_buffer::HEADER_SIZE;
use crate::{RingBuffer, RingBufferHeader};
use std::path::PathBuf;
use std::sync::atomic::Ordering;

impl RingBuffer {
    /// Get a reference to the buffer header
    #[inline(always)]
    pub(super) fn header(&self) -> &RingBufferHeader {
        unsafe { &*(self.mmap.as_ptr() as *const RingBufferHeader) }
    }

    /// Get a mutable reference to the buffer
    #[inline(always)]
    pub(super) fn buffer_ptr(&self) -> *const u8 {
        unsafe { self.mmap.as_ptr().add(HEADER_SIZE) }
    }

    /// Get a mutable pointer to the buffer
    #[inline(always)]
    pub(super) fn buffer_ptr_mut(&mut self) -> *mut u8 {
        unsafe { self.mmap.as_mut_ptr().add(HEADER_SIZE) }
    }

    /// Get the capacity of the buffer
    #[inline(always)]
    pub fn capacity(&self) -> u64 {
        self.header().capacity.load(Ordering::Relaxed)
    }

    /// Check if the buffer is empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        let read_pos = self.header().read_pos.load(Ordering::Acquire);
        let write_pos = self.header().write_pos.load(Ordering::Acquire);
        read_pos == write_pos
    }

    /// Check if the buffer is full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        let capacity = self.capacity();
        let read_pos = self.header().read_pos.load(Ordering::Acquire);
        let write_pos = self.header().write_pos.load(Ordering::Acquire);

        write_pos - read_pos == capacity
    }

    /// Get available data size
    #[inline(always)]
    pub fn available_data(&self) -> u64 {
        let read_pos = self.header().read_pos.load(Ordering::Acquire);
        let write_pos = self.header().write_pos.load(Ordering::Acquire);

        write_pos - read_pos
    }

    /// Get available space
    #[inline(always)]
    pub fn available_space(&self) -> u64 {
        let capacity = self.capacity();
        let read_pos = self.header().read_pos.load(Ordering::Acquire);
        let write_pos = self.header().write_pos.load(Ordering::Acquire);

        capacity - (write_pos - read_pos)
    }

    /// Get the path of the ring buffer
    #[inline(always)]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Get delete_on_drop setting
    #[inline(always)]
    pub fn delete_on_drop(&self) -> bool {
        self.delete_on_drop
    }
}
