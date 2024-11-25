// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . The DeepCausality Authors. All Rights Reserved.
#[cfg(feature = "unsafe")]
use crate::WindowStorage;

#[cfg(feature = "unsafe")]
const ERROR_EMPTY_ARRAY: &str = "Array is empty";
#[cfg(feature = "unsafe")]
const ERROR_ARRAY_NOT_FILLED: &str = "Array is not yet filled";

#[cfg(feature = "unsafe")]
#[repr(C, align(64))]
#[derive(Debug)]
pub struct UnsafeArrayStorage<T, const SIZE: usize, const CAPACITY: usize>
where
    T: PartialEq + Copy + Default,
    [T; CAPACITY]: Sized,
{
    arr: [T; CAPACITY],
    head: usize,
    tail: usize,
    size: usize,
}

#[cfg(feature = "unsafe")]
impl<T, const SIZE: usize, const CAPACITY: usize> UnsafeArrayStorage<T, SIZE, CAPACITY>
where
    T: PartialEq + Copy + Default,
    [T; CAPACITY]: Sized,
{
    #[inline(always)]
    pub fn new() -> Self {
        assert!(CAPACITY > SIZE, "CAPACITY must be greater than SIZE");
        Self {
            arr: [T::default(); CAPACITY],
            head: 0,
            tail: 0,
            size: SIZE,
        }
    }

    #[inline(always)]
    fn filled(&self) -> bool {
        unsafe { self.tail.unchecked_sub(self.head) >= self.size }
    }

    #[inline(always)]
    fn rewind(&mut self) {
        unsafe {
            let type_size = std::mem::size_of::<T>();
            let src = self.arr.as_ptr().add(self.tail - self.size);
            let dst = self.arr.as_mut_ptr();

            if type_size >= 4 {
                // For 4+ byte types, use optimized copying
                let bytes_to_copy = self.size * type_size;
                let chunks_16 = bytes_to_copy / 16;
                let remainder = bytes_to_copy % 16;

                // Copy 16-byte chunks
                if chunks_16 > 0 {
                    let src_bytes = src as *const u8;
                    let dst_bytes = dst as *mut u8;
                    for i in 0..chunks_16 {
                        std::ptr::copy_nonoverlapping(
                            src_bytes.add(i * 16),
                            dst_bytes.add(i * 16),
                            16,
                        );
                    }
                }

                // Copy remaining bytes
                if remainder > 0 {
                    let src_bytes = (src as *const u8).add(chunks_16 * 16);
                    let dst_bytes = (dst as *mut u8).add(chunks_16 * 16);
                    std::ptr::copy_nonoverlapping(src_bytes, dst_bytes, remainder);
                }
            } else {
                // Fall back to standard copy for smaller types
                std::ptr::copy_nonoverlapping(src, dst, self.size);
            }
        }
        self.head = 0;
        self.tail = self.size;
    }
}

#[cfg(feature = "unsafe")]
impl<T, const SIZE: usize, const CAPACITY: usize> Default for UnsafeArrayStorage<T, SIZE, CAPACITY>
where
    T: PartialEq + Copy + Default,
    [T; SIZE]: Sized,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "unsafe")]
impl<T, const SIZE: usize, const CAPACITY: usize> WindowStorage<T>
    for UnsafeArrayStorage<T, SIZE, CAPACITY>
where
    T: PartialEq + Copy + Default,
    [T; SIZE]: Sized,
{
    #[inline(always)]
    fn push(&mut self, value: T) {
        unsafe {
            if self.tail >= CAPACITY {
                self.rewind();
            }

            *self.arr.get_unchecked_mut(self.tail) = value;
            self.tail = self.tail.wrapping_add(1);

            if self.tail.unchecked_sub(self.head) > self.size {
                self.head = self.tail.unchecked_sub(self.size);
            }
        }
    }

    #[inline(always)]
    fn first(&self) -> Result<T, String> {
        if self.tail == 0 {
            return Err(ERROR_EMPTY_ARRAY.to_string());
        }
        unsafe { Ok(*self.arr.get_unchecked(self.head)) }
    }

    #[inline(always)]
    fn last(&self) -> Result<T, String> {
        if !self.filled() {
            return Err(ERROR_ARRAY_NOT_FILLED.to_string());
        }
        unsafe { Ok(*self.arr.get_unchecked(self.tail - 1)) }
    }

    #[inline(always)]
    fn tail(&self) -> usize {
        self.tail
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.size
    }

    #[inline(always)]
    fn get_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(
                self.arr.as_ptr().add(self.head),
                self.tail.saturating_sub(self.head).min(self.size),
            )
        }
    }
}
