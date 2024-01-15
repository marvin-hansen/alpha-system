pub fn abs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

pub fn min(numbers: &[f64]) -> f64 {
    let mut number = numbers[0];
    for item in 0..numbers.len() {
        if numbers[item] < number {
            number = numbers[item]
        }
    }
    number
}

pub fn max(numbers: &[f64]) -> f64 {
    let mut number = numbers[0];
    for item in 0..numbers.len() {
        if numbers[item] > number {
            number = numbers[item]
        }
    }
    number
}
