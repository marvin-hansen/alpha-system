
pub fn abs(
    x: f64
)
    -> f64
{
    if x < 0.0 {
        -x
    } else {
        x
    }
}

pub fn min(
    numbers: &[f64]
)
    -> f64
{
    let mut number = numbers[0];
    for i in 0..numbers.len() {
        if numbers[i] < number {
            number = numbers[i]
        }
    }
    number
}

pub fn max(
    numbers: &[f64]
)
    -> f64
{
    let mut number = numbers[0];
    for i in 0..numbers.len() {
        if numbers[i] > number {
            number = numbers[i]
        }
    }
    number
}