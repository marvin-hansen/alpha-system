/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crypto_utils::RelaxedAtomicCounter;

#[test]
fn test_increment_and_get() {
    let counter = RelaxedAtomicCounter::new();
    assert_eq!(counter.increment_and_get(), 1);
    assert_eq!(counter.increment_and_get(), 2);
    assert_eq!(counter.increment_and_get(), 3);
}

#[test]
fn test_with_start_value() {
    let counter = RelaxedAtomicCounter::with_start_value(10);
    assert_eq!(counter.increment_and_get(), 11);
    assert_eq!(counter.increment_and_get(), 12);
}

#[test]
fn test_get_counter() {
    let counter = RelaxedAtomicCounter::new();
    assert_eq!(counter.get_counter(), 0);
    counter.increment_and_get();
    assert_eq!(counter.get_counter(), 1);
    counter.increment_and_get();
    assert_eq!(counter.get_counter(), 2);
}
