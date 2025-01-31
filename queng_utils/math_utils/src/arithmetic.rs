/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

#[must_use]
pub fn abs(x: f64) -> f64 {
    x.abs()
}

#[allow(clippy::needless_range_loop)]
#[must_use]
pub fn min(numbers: &[f64]) -> f64 {
    let mut number = numbers[0];
    for i in 0..numbers.len() {
        if numbers[i] < number {
            number = numbers[i];
        }
    }
    number
}

#[allow(clippy::needless_range_loop)]
#[must_use]
pub fn max(numbers: &[f64]) -> f64 {
    let mut number = numbers[0];
    for i in 0..numbers.len() {
        if numbers[i] > number {
            number = numbers[i];
        }
    }
    number
}
