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
    size: usize,
    head: usize,
    tail: usize,
}

#[cfg(feature = "unsafe")]
impl<T, const SIZE: usize, const CAPACITY: usize> UnsafeArrayStorage<T, SIZE, CAPACITY>
where
    T: PartialEq + Copy + Default,
    [T; CAPACITY]: Sized,
{
    /// Creates a new UnsafeArrayStorage instance
    ///
    /// # Implementation Notes
    /// - Initializes array with default values
    /// - Requires 4-byte alignment for optimal performance
    #[inline(always)]
    pub fn new() -> Self {
        assert!(CAPACITY > SIZE, "CAPACITY must be greater than SIZE");
        Self {
            arr: [T::default(); CAPACITY],
            size: SIZE,
            head: 0,
            tail: 0,
        }
    }

    /// Checks if the sliding window is filled to its maximum size
    #[inline(always)]
    fn filled(&self) -> bool {
        self.tail - self.head >= self.size
    }

    /// Rewinds the storage by copying elements to array start
    ///
    /// # Implementation Notes
    /// - Copies the last SIZE elements to the beginning
    #[inline(always)]
    fn rewind(&mut self) {
        // Copy the last SIZE elements to the beginning
        self.arr.copy_within(self.tail - self.size..self.tail, 0);
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
    /// Creates a default UnsafeArrayStorage instance
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
    /// Pushes a new value into storage
    ///
    /// # Args
    /// * `value` - Value to push
    ///
    /// # Implementation Notes
    /// - Automatically rewinds when full
    /// - Adjusts head to maintain window size
    #[inline(always)]
    fn push(&mut self, value: T) {
        // Rewind if there's not enough space for the next element
        if self.tail + 1 >= CAPACITY {
            self.rewind();
        }

        // Store the value and update tail
        self.arr[self.tail] = value;
        self.tail += 1;

        // Update head if window size exceeded
        if self.tail - self.head > self.size {
            self.head = self.tail - self.size;
        }
    }

    /// Returns first element in window
    ///
    /// # Errors
    /// Returns error if storage is empty
    ///
    /// # Implementation Notes
    /// - Handles both normal and wrapped states
    #[inline(always)]
    fn first(&self) -> Result<T, String> {
        if self.tail == 0 {
            return Err(ERROR_EMPTY_ARRAY.to_string());
        }

        Ok(self.arr[self.head])
    }

    /// Returns last element in window
    ///
    /// # Errors
    /// Returns error if storage not filled
    ///
    /// # Implementation Notes
    /// - Verifies fill state before access
    #[inline(always)]
    fn last(&self) -> Result<T, String> {
        if !self.filled() {
            return Err(ERROR_ARRAY_NOT_FILLED.to_string());
        }
        Ok(self.arr[self.tail - 1])
    }

    /// Returns current tail position
    #[inline(always)]
    fn tail(&self) -> usize {
        self.tail
    }

    /// Returns window size
    #[inline(always)]
    fn size(&self) -> usize {
        self.size
    }

    /// Returns slice of current window contents
    ///
    /// # Implementation Notes
    /// - Handles both normal and wrapped states
    #[inline(always)]
    fn get_slice(&self) -> &[T] {
        &self.arr[self.head..self.tail.min(self.head + self.size)]
    }
}
