use sliding_window::{SlidingWindow, UnsafeArrayStorage};

fn main() {
    // Size refers to the maximum number of elements the sliding window can store
    const SIZE: usize = 4;
    // Capacity refers to the maximum number of elements before a rewind occurs
    const CAPACITY: usize = 12;

    println!("Creating window...");

    // Create type alias for better readability
    type Window = SlidingWindow<UnsafeArrayStorage<u32, SIZE, CAPACITY>, u32>;

    // Create a new sliding window with unsafe array storage for better performance
    let mut window: Window = sliding_window::new_with_unsafe_array_storage();

    println!("Window created. Pushing values...");

    // Push some values
    window.push(100);
    window.push(200);
    window.push(300);
    window.push(400);

    println!("Values pushed. Checking if filled...");

    // Window is now filled
    assert!(window.filled());

    println!("Window is filled. Getting values...");

    // Get values in different formats
    println!("First value: {:?}", window.first());
    println!("Last value: {:?}", window.last());
    println!("As vec: {:?}", window.vec());

    println!("Pushing more values...");

    // Push more values - older values will be dropped
    window.push(500);
    window.push(600);

    println!("After pushing more values:");
    println!("As vec: {:?}", window.vec());
}
