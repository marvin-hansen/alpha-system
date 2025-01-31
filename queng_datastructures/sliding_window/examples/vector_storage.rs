/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use sliding_window::{SlidingWindow, VectorStorage};

fn main() {
    // Size refers to the maximum number of elements the sliding window can store
    const SIZE: usize = 4;
    // Multiplier to calculate capacity as size * multiple
    const MULT: usize = 3; // Capacity: 4 * 3 = 12

    // Create a sliding window for f64 values
    let mut window: SlidingWindow<VectorStorage<f64>, f64> =
        sliding_window::new_with_vector_storage(SIZE, MULT);

    // Push some values
    window.push(1.0);
    window.push(2.0);
    window.push(3.0);
    window.push(4.0);

    // Window is now filled
    assert!(window.filled());

    // Get values in different formats
    println!("First value: {:?}", window.first());
    println!("Last value: {:?}", window.last());
    println!("As vector: {:?}", window.vec());

    // Push more values - older values will be dropped
    window.push(5.0);
    window.push(6.0);

    println!("After pushing more values:");
    println!("As vector: {:?}", window.vec());
}
