use sliding_window::{storage_unsafe::UnsafeArrayStorage, SlidingWindow};

#[test]
fn test_drop_old_values() {
    const SIZE: usize = 3;
    const CAPACITY: usize = 6;
    let mut window: SlidingWindow<UnsafeArrayStorage<i32, SIZE, CAPACITY>, i32> =
        sliding_window::new_with_unsafe_array_storage();

    // Fill the window
    window.push(1);
    window.push(2);
    window.push(3);
    assert!(window.filled());
    assert_eq!(window.vec().unwrap(), vec![1, 2, 3]);

    // Push more values, older values should be dropped
    window.push(4);
    assert_eq!(window.vec().unwrap(), vec![2, 3, 4]);

    window.push(5);
    assert_eq!(window.vec().unwrap(), vec![3, 4, 5]);

    window.push(6);
    assert_eq!(window.vec().unwrap(), vec![4, 5, 6]);

    // Verify first and last values
    assert_eq!(window.first().unwrap(), 4);
    assert_eq!(window.last().unwrap(), 6);

    // Push enough values to trigger a rewind
    window.push(7);
    window.push(8);
    window.push(9);
    assert_eq!(window.vec().unwrap(), vec![7, 8, 9]);
}
