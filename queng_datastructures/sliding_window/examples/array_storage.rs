/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use sliding_window::{ArrayStorage, SlidingWindow};

fn main() {
    // Size refers to the maximum number of elements the sliding window can store
    const SIZE: usize = 4;
    // Capacity refers to the maximum number of elements before a rewind occurs
    const CAPACITY: usize = 12;

    // Create type alias for better readability
    type Window = SlidingWindow<ArrayStorage<i32, SIZE, CAPACITY>, i32>;

    // Create a new sliding window
    let mut window: Window = sliding_window::new_with_array_storage();

    // Push some values
    window.push(10);
    window.push(20);
    window.push(30);
    window.push(40);

    // Window is now filled
    assert!(window.filled());

    // Get values in different formats
    println!("First value: {:?}", window.first());
    println!("Last value: {:?}", window.last());
    println!("As array: {:?}", window.slice());

    // Push more values - older values will be dropped
    window.push(50);
    window.push(60);

    println!("After pushing more values:");
    println!("As array: {:?}", window.slice());
}
