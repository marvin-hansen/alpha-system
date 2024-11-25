pub const SIZE: usize = 10;
#[cfg(feature = "unsafe")]
pub const CAPACITY: usize = MULT * SIZE;
pub const MULT: usize = 100;
